fn main() {
    println!("cargo:rerun-if-changed=src/main.cpp");

    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        cc::Build::new()
            .cpp(true)
            .file("src/main.cpp")
            .flag("/GS-")
            .flag("/O1")
            .compile("runner_cpp");

        println!("cargo:rustc-link-arg=/NODEFAULTLIB");
        println!("cargo:rustc-link-arg=/ENTRY:mainEntryPoint");
        println!("cargo:rustc-link-arg=/MERGE:.rdata=.text");
        println!("cargo:rustc-link-arg=/SECTION:.text,EWR");
        println!("cargo:rustc-link-arg=/ALIGN:16");
        println!("cargo:rustc-link-arg=/FILEALIGN:16");
        println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");

        println!("cargo:rustc-link-lib=kernel32");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=gdi32");
    }
}