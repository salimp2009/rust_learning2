fn main() {
    println!("cargo:rerun-if-change=src/hello_world.c");
    cc::Build::new()
        .file("src/hello_world.c")
        .compile("hello_world");
}
