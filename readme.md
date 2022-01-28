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