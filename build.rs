extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/merlin.c")
        .include("src")
        .define("HAVE_EXPLICIT_BZERO", "TRUE")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-O2")
        .compile("merlin");
}
