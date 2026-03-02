use std::env;
use std::fs::{self, OpenOptions};
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_SPACETIME_URI: &str = "http://127.0.0.1:3000";
const DEFAULT_SPACETIME_DB: &str = "rpg-raid-shop-dev";
const DEFAULT_WASM_PATH: &str = "target/wasm32-unknown-unknown/release/spacetimedb_module.wasm";

fn main() -> ExitCode {
    if let Err(error) = run() {
        eprintln!("[xtask][error] {error}");
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_usage();
        return Ok(());
    }

    let repo_root = repo_root()?;

    match args[0].as_str() {
        "dev-up" => dev_up(&repo_root),
        "dev-client" => {
            let guest = args.get(1).cloned().unwrap_or_else(default_guest_name);
            client_run(&repo_root, &guest)
        }
        "dev-down" => dev_down(&repo_root),
        "db" => run_db(&repo_root, &args[1..]),
        "client" => run_client(&repo_root, &args[1..]),
        "help" | "-h" | "--help" => {
            print_usage();
            Ok(())
        }
        unknown => Err(format!("unknown command: {unknown}")),
    }
}

fn run_db(repo_root: &Path, args: &[String]) -> Result<(), String> {
    let subcommand = args.first().map(String::as_str).unwrap_or("help");
    match subcommand {
        "start" => db_start(repo_root),
        "publish" => db_publish(repo_root),
        "generate" => db_generate(repo_root),
        "sync" => {
            db_publish(repo_root)?;
            db_generate(repo_root)
        }
        "config" => {
            db_config(repo_root)?;
            Ok(())
        }
        "help" | "-h" | "--help" => {
            print_db_usage();
            Ok(())
        }
        unknown => Err(format!("unknown db command: {unknown}")),
    }
}

fn run_client(repo_root: &Path, args: &[String]) -> Result<(), String> {
    let subcommand = args.first().map(String::as_str).unwrap_or("help");
    match subcommand {
        "run" => {
            let guest = args.get(1).cloned().unwrap_or_else(default_guest_name);
            client_run(repo_root, &guest)
        }
        "help" | "-h" | "--help" => {
            print_client_usage();
            Ok(())
        }
        unknown => Err(format!("unknown client command: {unknown}")),
    }
}

fn db_start(repo_root: &Path) -> Result<(), String> {
    let spacetime = spacetime_bin()?;
    run_command(
        Command::new(spacetime)
            .current_dir(repo_root)
            .arg("start")
            .arg("--listen-addr")
            .arg("0.0.0.0:3000")
            .arg("--in-memory")
            .arg("--non-interactive"),
    )
}

fn dev_up(repo_root: &Path) -> Result<(), String> {
    ensure_db_running_background(repo_root)?;
    db_publish(repo_root)?;
    db_generate(repo_root)
}

fn dev_down(repo_root: &Path) -> Result<(), String> {
    let pid_file = pid_file(repo_root);
    if !pid_file.exists() {
        println!("No managed DB pid file found at {}", pid_file.display());
        return Ok(());
    }

    let pid_text = fs::read_to_string(&pid_file).map_err(|error| error.to_string())?;
    let pid = pid_text
        .trim()
        .parse::<u32>()
        .map_err(|error| format!("invalid pid file contents: {error}"))?;

    let status = Command::new("kill")
        .arg(pid.to_string())
        .status()
        .map_err(|error| error.to_string())?;
    if !status.success() {
        return Err(format!("failed to stop DB process pid={pid} (status {status})"));
    }

    fs::remove_file(&pid_file).map_err(|error| error.to_string())?;
    println!("Stopped DB process pid={pid}");
    Ok(())
}

fn db_publish(repo_root: &Path) -> Result<(), String> {
    let spacetime = spacetime_bin()?;
    let db = env_or_default("SPACETIME_DB", DEFAULT_SPACETIME_DB);
    let uri = env_or_default("SPACETIME_URI", DEFAULT_SPACETIME_URI);

    run_command(
        Command::new(spacetime)
            .current_dir(repo_root)
            .arg("publish")
            .arg(db)
            .arg("-s")
            .arg(uri)
            .arg("-y")
            .arg("--module-path")
            .arg("crates/spacetimedb_module"),
    )
}

fn db_generate(repo_root: &Path) -> Result<(), String> {
    let spacetime = spacetime_bin()?;
    let wasm_path = env_or_default("WASM_PATH", DEFAULT_WASM_PATH);

    if !repo_root.join(&wasm_path).exists() {
        run_command(
            Command::new("cargo")
                .current_dir(repo_root)
                .arg("build")
                .arg("--release")
                .arg("-p")
                .arg("spacetimedb_module")
                .arg("--target")
                .arg("wasm32-unknown-unknown"),
        )?;
    }

    run_command(
        Command::new(spacetime)
            .current_dir(repo_root)
            .arg("generate")
            .arg("--lang")
            .arg("rust")
            .arg("--out-dir")
            .arg("crates/client_bevy/src/module_bindings")
            .arg("--bin-path")
            .arg(wasm_path)
            .arg("-y"),
    )
}

fn db_config(repo_root: &Path) -> Result<(), String> {
    let spacetime = spacetime_bin()?;
    let db = env_or_default("SPACETIME_DB", DEFAULT_SPACETIME_DB);
    let uri = env_or_default("SPACETIME_URI", DEFAULT_SPACETIME_URI);
    let wasm_path = env_or_default("WASM_PATH", DEFAULT_WASM_PATH);

    println!("SPACETIME_BIN={spacetime}");
    println!("SPACETIME_URI={uri}");
    println!("SPACETIME_DB={db}");
    println!("WASM_PATH={wasm_path}");
    println!("REPO_ROOT={}", repo_root.display());

    Ok(())
}

fn client_run(repo_root: &Path, guest_name: &str) -> Result<(), String> {
    let db = env_or_default("SPACETIME_DB", DEFAULT_SPACETIME_DB);
    let uri = env_or_default("SPACETIME_URI", DEFAULT_SPACETIME_URI);

    run_command(
        Command::new("cargo")
            .current_dir(repo_root)
            .arg("run")
            .arg("-p")
            .arg("client_bevy")
            .env("SPACETIME_URI", uri)
            .env("SPACETIME_DB", db)
            .env("SPACETIME_GUEST", guest_name),
    )
}

fn ensure_db_running_background(repo_root: &Path) -> Result<(), String> {
    let spacetime = spacetime_bin()?;
    let uri = env_or_default("SPACETIME_URI", DEFAULT_SPACETIME_URI);
    let (host, port) = host_port_from_uri(&uri)?;

    if is_port_open(&host, port) {
        println!("SpacetimeDB already reachable at {host}:{port}");
        return Ok(());
    }

    let run_dir = repo_root.join("target/dev");
    fs::create_dir_all(&run_dir).map_err(|error| error.to_string())?;
    let log_file_path = run_dir.join("spacetime.log");
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .map_err(|error| error.to_string())?;
    let log_file_err = log_file.try_clone().map_err(|error| error.to_string())?;

    let child = Command::new(spacetime)
        .current_dir(repo_root)
        .arg("start")
        .arg("--listen-addr")
        .arg("0.0.0.0:3000")
        .arg("--in-memory")
        .arg("--non-interactive")
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(log_file_err))
        .spawn()
        .map_err(|error| error.to_string())?;

    let pid = child.id();
    let pid_file = pid_file(repo_root);
    fs::write(&pid_file, pid.to_string()).map_err(|error| error.to_string())?;

    for _ in 0..40 {
        if is_port_open(&host, port) {
            println!(
                "Started SpacetimeDB in background (pid={pid}) at {host}:{port}; logs: {}",
                log_file_path.display()
            );
            return Ok(());
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }

    Err(format!(
        "SpacetimeDB did not become ready at {host}:{port}; check logs at {}",
        log_file_path.display()
    ))
}

fn pid_file(repo_root: &Path) -> PathBuf {
    repo_root.join("target/dev/spacetime.pid")
}

fn host_port_from_uri(uri: &str) -> Result<(String, u16), String> {
    let no_scheme = uri
        .strip_prefix("http://")
        .or_else(|| uri.strip_prefix("https://"))
        .unwrap_or(uri);
    let host_port = no_scheme
        .split('/')
        .next()
        .ok_or_else(|| "invalid SPACETIME_URI".to_string())?;

    if let Some((host, port)) = host_port.rsplit_once(':') {
        let parsed = port
            .parse::<u16>()
            .map_err(|error| format!("invalid port in SPACETIME_URI: {error}"))?;
        Ok((host.to_string(), parsed))
    } else {
        Ok((host_port.to_string(), 80))
    }
}

fn is_port_open(host: &str, port: u16) -> bool {
    let address = format!("{host}:{port}");
    let Some(socket) = address
        .to_socket_addrs()
        .ok()
        .and_then(|mut addresses| addresses.next())
    else {
        return false;
    };

    TcpStream::connect_timeout(&socket, std::time::Duration::from_millis(300)).is_ok()
}

fn run_command(command: &mut Command) -> Result<(), String> {
    let status = command.status().map_err(|error| error.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed with status {status}"))
    }
}

fn spacetime_bin() -> Result<String, String> {
    if let Ok(path) = env::var("SPACETIME_BIN") {
        return Ok(path);
    }

    if command_exists("spacetime") {
        return Ok("spacetime".to_string());
    }

    let local = format!("{}/.local/bin/spacetime", env::var("HOME").unwrap_or_default());
    if Path::new(&local).exists() {
        return Ok(local);
    }

    Err("SpacetimeDB CLI not found. Set SPACETIME_BIN or add 'spacetime' to PATH.".to_string())
}

fn command_exists(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn env_or_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn repo_root() -> Result<PathBuf, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .ok_or_else(|| "unable to resolve repository root".to_string())
}

fn default_guest_name() -> String {
    let host = env::var("HOSTNAME").unwrap_or_else(|_| "client".to_string());
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    format!("Guest_{host}_{seconds}")
}

fn print_usage() {
    println!("Usage: cargo run -p xtask -- <command>");
    println!("Lifecycle commands:");
    println!("  dev-up");
    println!("  dev-client [GuestName]");
    println!("  dev-down");
    println!();
    println!("Low-level commands:");
    println!("  db <subcommand>");
    println!("  client <subcommand>");
    println!("Run `cargo run -p xtask -- db help` for DB commands.");
    println!("Run `cargo run -p xtask -- client help` for client commands.");
}

fn print_db_usage() {
    println!("DB commands:");
    println!("  db start");
    println!("  db publish");
    println!("  db generate");
    println!("  db sync");
    println!("  db config");
}

fn print_client_usage() {
    println!("Client commands:");
    println!("  client run [GuestName]");
}
