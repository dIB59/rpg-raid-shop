# rpg-raid-shop

A [Bevy](https://bevyengine.org/) game with **hot-reloadable systems** via
[`bevy_simple_subsecond_system`](https://crates.io/crates/bevy_simple_subsecond_system).

Edit the body of any system marked `#[hot]` while the game is running, hit save,
and the change applies live — no restart, no lost game state.

## Prerequisites

- Rust (stable)
- The Dioxus CLI, which provides the `dx` hot-patch runner:

  ```sh
  cargo install dioxus-cli@0.7.0-rc.1
  ```

  (Any `dx` 0.7.x works.)

## Run with hot reloading

```sh
BEVY_ASSET_ROOT="." dx serve --hot-patch
```

Then open `src/main.rs`, tweak the math inside `animate_player` (e.g. change
`* 200.0` to `* 400.0`), save, and watch the sprite update instantly.

For faster compiles during development, dynamically link Bevy:

```sh
BEVY_ASSET_ROOT="." dx serve --hot-patch --features dev
```

## Run normally (no hot reload)

```sh
cargo run
```

## How it works

- `SimpleSubsecondPlugin` is added in `main()`.
- Systems you want to edit live are annotated with `#[hot]`.
- Only the **body** of a `#[hot]` system is hot-patched. Changing its
  signature (params), adding/removing systems, or editing `setup` needs a
  restart. For setup-style systems use `#[hot(rerun_on_hot_patch = true)]`.
