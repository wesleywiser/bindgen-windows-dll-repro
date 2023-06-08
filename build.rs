use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("./cpp/example.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let cpp_dir = env::current_dir().unwrap().join("cpp");
    println!("cargo:rustc-link-search=native={}", cpp_dir.display());
    println!("cargo:rustc-link-lib=dylib=example");
}