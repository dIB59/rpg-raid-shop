install:
    cargo install --locked dioxus-cli@0.7.9

# Run the game with hot patching enabled
dev:
    BEVY_ASSET_ROOT="." dx serve --hot-patch
