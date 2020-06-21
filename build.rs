extern crate bindgen;
extern crate cc;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if !Path::new("nuklear-c/nuklear/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "--recursive"])
            .status()
            .unwrap();
    }
    cc::Build::new()
        .file("nuklear-c/bind.c")
        .compile("libnuklear");
    let bindings = bindgen::Builder::default()
        .header("nuklear-c/nuklear_use.h")
        .generate()
        .unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
