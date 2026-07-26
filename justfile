install:
    cargo install --locked dioxus-cli@0.7.9

# Run the game with hot patching enabled
dev:
    BEVY_ASSET_ROOT="." dx serve --hot-patch --features bevy/hotpatching

# Hot patching + dynamically linked Bevy for faster rebuilds
dev-fast:
    BEVY_ASSET_ROOT="." dx serve --hot-patch --features bevy/hotpatching --features dev
