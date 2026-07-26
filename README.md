# rpg-raid-shop

A [Bevy](https://bevyengine.org/) game using Bevy's built-in **hot patching**
(the `hotpatching` cargo feature, powered by Dioxus's subsecond).

Edit the body of any system while the game is running, hit save, and the
change applies live — no restart, no lost game state.

## Prerequisites

- Rust (stable)
- The Dioxus CLI, which provides the `dx` hot-patch runner:

  ```sh
  cargo install --locked dioxus-cli@0.7.9
  ```

  (Any `dx` 0.7.x works.)

## Run with hot reloading

```sh
just dev
# or directly:
BEVY_ASSET_ROOT="." dx serve --hot-patch --features bevy/hotpatching
```

Then open `src/player.rs`, tweak the math inside `move_player`, save, and
watch the change apply instantly.

For faster rebuilds, dynamically link Bevy as well:

```sh
just dev-fast
```

## Run normally (no hot reload)

```sh
cargo run
```

The `hotpatching` feature is only passed on the `dx serve` command line, so
normal and release builds carry none of the hot-patching machinery.

## How it works

- With `bevy/hotpatching` enabled, **every system** is hot-patchable — no
  attribute or plugin needed.
- Only system **bodies** are hot-patched. Changing a system's signature
  (params), adding/removing systems, or editing `setup` needs a restart.
- Limitations: only code in the binary crate is patched, patched systems run
  as exclusive systems (no parallelism), and Wasm is not supported.
