fn main() {
    cc::Build::new()
        .file("./src/get_key.c")
        .compile("get_key_c");
}
