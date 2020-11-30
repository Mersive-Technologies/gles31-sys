extern crate bindgen;

use std::env;
use std::path::{PathBuf, Path};

fn main() {
    let android_home = env::var("ANDROID_HOME").expect("ANDROID_HOME not set!");
    let ndk_include_dir = Path::new(android_home.as_str()).join("ndk/21.3.6528147/sysroot/usr/include/");
    let gl31h = ndk_include_dir.join("GLES3/gl31.h");
    let gl31h = gl31h.to_str().unwrap();
    let ndk_include_dir = ndk_include_dir.to_str().unwrap();

    println!("cargo:rustc-link-lib=GLESv3");
    println!("cargo:rerun-if-changed={}", gl31h);

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", ndk_include_dir))
        .header(gl31h)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
