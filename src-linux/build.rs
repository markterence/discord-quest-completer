fn main() {
    println!("cargo:rerun-if-changed=src/main.c");

    cc::Build::new()
        .file("src/main.c")
        .opt_level(3)
        .compile("runner_c");

    println!("cargo:rustc-link-lib=c");
}
