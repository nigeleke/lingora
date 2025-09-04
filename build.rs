fn main() {
    // Defaults required for testing...
    println!("cargo::rustc-env=LANG=en_US.UTF-8");
    println!("cargo::rustc-env=LC_ALL=en_US.UTF-8");
}
