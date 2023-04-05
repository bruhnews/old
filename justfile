# Run in *dev mode*, serve files on localhost, reload client on file changes
serve:
    cargo watch -x "run -- --dev" -w src -w Cargo.toml

# Build for production
build:
    cargo run --no-default-features
