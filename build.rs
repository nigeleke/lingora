use std::env;

fn main() {
    if env::var("CARGO_CFG_TEST").is_ok() {
        println!("cargo:rustc-env=LANG=en_US.UTF-8");
        println!("cargo:rustc-env=LC_ALL=en_US.UTF-8");
    }
}
