# OpenGL ES 3.1 bindings for rust

Provide OpenGL ES 3.1 bindings for rust

## Building for iOS

```shell
rustup target add aarch64-apple-ios x86_64-apple-ios
cargo install cargo-lipo
cargo lipo
```

## Building for Android on MacOS

```shell
brew install --cask android-sdk
export ANDROID_HOME=$HOME/Library/Android/sdk/ 
cargo build
```
                      
## Publishing

```shell
cargo publish --target=aarch64-linux-android --no-verify
```

`--no-verify` is necessary because otherwise we get:

```text
  Source directory was modified by build.rs during cargo publish. Build scripts should not modify anything outside of OUT_DIR.
  Added: /Users/bgardner/workspace/gles31-sys/target/package/gles31-sys-0.2.0/src/bindings.rs
```

## TODO

1. Have CI tag and auto-increment merges to master
2. Have CI automatically publish to crates.io
3. Determing the correct way to publish without having to use `--no-verify`