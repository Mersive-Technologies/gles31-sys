extern crate bindgen;

use std::path::{Path, PathBuf};
use std::{env, fs};

/// Xcode framework includes work a bit weird so we need to create this symlink.
/// Otherwise OpenGLES/OpenGLESAvailability.h include will be not visible to bindgen.
/// If anyone knows how to make it more gracefully I will change it.
fn symlink_gles(include_dir: &Path) {
    let gles_dir = Path::new("temp/OpenGLES");

    if gles_dir.exists() {
        return;
    }

    fs::create_dir("temp").unwrap();
    #[cfg(target_os = "macos")]
    std::os::unix::fs::symlink(include_dir, gles_dir).unwrap();
}

fn ios_setup() {
    let framework_dir = Path::new("/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk/System/Library/Frameworks/OpenGLES.framework/");
    let include_dir = framework_dir.join("Headers");

    symlink_gles(&include_dir);

    let gl31h = include_dir.join("ES3/gl.h");
    let gl31h = gl31h.to_str().unwrap();
    let include_dir = include_dir.to_str().unwrap();

    generate_bindings(gl31h, &[include_dir, "temp"]);
}

fn android_setup() {
    let ndk_ver = env::var("NDK_VER").unwrap_or("21.3.6528147".to_string());
    let android_home = env::var("ANDROID_HOME").expect("ANDROID_HOME not set!");
    let path = format!(
        "ndk/{}/sysroot/usr/include",
        ndk_ver
    );
    let ndk_include_dir = Path::new(android_home.as_str()).join(path);
    let gl31h = ndk_include_dir.join("GLES3/gl31.h");
    let gl31h = gl31h.to_str().unwrap();
    let ndk_include_dir = ndk_include_dir.to_str().unwrap();

    generate_bindings(gl31h, &[ndk_include_dir]);
}

fn generate_bindings(gl31h: &str, includes: &[&str]) {
    println!("cargo:rustc-link-lib=GLESv3");
    println!("cargo:rerun-if-changed={}", gl31h);

    let mut bindings = bindgen::Builder::default();

    for include in includes {
        println!("{}", include);
        bindings = bindings.clang_arg(format!("-I{}", include));
    }

    let bindings = bindings
        .header(gl31h)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS") == Ok("android".into()) {
        android_setup()
    } else if std::env::var("CARGO_CFG_TARGET_OS") == Ok("ios".into()) {
        ios_setup()
    } else {
        panic!("Unsupported target OS. Only iOS and Android supported.")
    }
}
