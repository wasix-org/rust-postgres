use std::env;

const WASI: &str = "wasi";

fn main() {
    // let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if [WASI].contains(&os.as_str()) {
        println!("cargo:rustc-link-lib=static=clang_rt.builtins-wasm32");
        println!("cargo:rustc-link-search=native={}/wasm-libs", src_dir);
    }
}
