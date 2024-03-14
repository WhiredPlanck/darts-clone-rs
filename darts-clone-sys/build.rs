use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=darts");

    let mut build = cc::Build::new();
    build
        .files(["libdarts/src/darts.cc"])
        .include("libdarts/src")
        .define("VERSION", "0.3.2")
        .define("PACKAGE_NAME", "libdarts")
        .cpp(true);

    build.compile("darts");
}
