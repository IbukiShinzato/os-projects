fn main() {
    println!("build /src/ffi/mmap_copy.c");
    cc::Build::new()
        .file("src/ffi/mmap_copy.c")
        .compile("mmap_copy");
}
