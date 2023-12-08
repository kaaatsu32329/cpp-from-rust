#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cpp-from-rust/include/print_hello.hpp");

        fn print_hello();
    }
}

fn main() {
    ffi::print_hello();
}
