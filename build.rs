extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/merlin.c")
        .include("src")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-O2")
        .compile("merlin");
}
