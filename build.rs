fn main() {
    cc::Build::new()
        .cpp(true)
        .files(vec!["src/print_hello.cpp"])
        .compile("cpp_from_rust");

    println!("cargo:rerun-if-changed=src/*.cpp")
}
