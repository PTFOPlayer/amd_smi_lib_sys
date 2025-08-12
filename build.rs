use std::{env, path::PathBuf};

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }
    println!(r"cargo:rustc-link-search=/opt/rocm/lib");
    println!("cargo:rustc-link-lib=amd_smi");

    let bindings = bindgen::Builder::default()
        .header("./amdsmi.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();

    let out_path = PathBuf::from("./src");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
