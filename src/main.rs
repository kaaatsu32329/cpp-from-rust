#![feature(extern_types)]

extern "C" {
    fn print_hello();
}

fn main() {
    unsafe {
        print_hello();
    }
}
