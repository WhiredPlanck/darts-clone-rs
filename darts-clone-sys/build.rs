use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:root={}", out_dir);
    let include_dir = env::current_dir().unwrap().join("libdarts/src");
    println!("cargo:include={}", include_dir.display());


    let mut build = cc::Build::new();
    build
        .files(["libdarts/src/darts.cc"])
        .define("VERSION", "0.3.2")
        .define("PACKAGE_NAME", "libdarts")
        .flag("-Wall");

    build.compile("darts");
}
