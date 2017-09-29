extern crate bindgen;
extern crate cc;

use std::path::PathBuf;
use std::env;

fn main() {
    cc::Build::new()
        .file("c/circle.c")
        .file("c/point.c")
        .compile("geom");

    let bindings = bindgen::Builder::default()
        .header("c/point.h")
        .header("c/circle.h")
        .generate()
        .expect("Couldn't generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
