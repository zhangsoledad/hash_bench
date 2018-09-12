// build.rs

extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/blake2b.c")
        .compile("libblake2b.a");
}
