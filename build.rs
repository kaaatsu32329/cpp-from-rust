fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/print_hello.cpp")
        .flag_if_supported("-std=c++20")
        .compile("cxxbridge-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/print_hello.cpp");
    println!("cargo:rerun-if-changed=include/print_hello.hpp");
}
