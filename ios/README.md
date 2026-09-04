# iOS host (Xcode / LiveContainer)

1. Sources under `ios/WowIos/Sources/`.
2. Link `libwow_client.a` from `cargo build -p wow-client --release --target aarch64-apple-ios`.
3. Metal + Security frameworks.
4. UIFileSharingEnabled so Documents appears in Files.
