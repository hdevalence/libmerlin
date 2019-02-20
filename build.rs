extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/merlin.c")
        .include("src")
        .flag("-O2")
        .compile("merlin");
}
