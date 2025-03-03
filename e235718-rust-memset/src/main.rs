use libc::{free, malloc, size_t};
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn main() {
    // local data
    let mut file = File::create("data/malloc_data_macos.csv").expect("Failed to create file");

    // amane data
    // let mut file = File::create("data/malloc_data_linux.csv").expect("Failed to create file");

    writeln!(file, "MemorySize,Time(ns)").expect("Failed to write to file");

    // define memory size
    let memory_sizes: Vec<usize> = vec![
        1,
        10,
        100,
        1000,
        10000,
        1000000,
        10000000,
        100000000,
        1000000000,
        10000000000,
        100000000000,
    ];

    for &memory_size in &memory_sizes {
        let start = Instant::now();

        // memset
        unsafe {
            let ptr = malloc(memory_size as size_t);
            if ptr.is_null() {
                eprintln!("Failed to allocate memory of size: {}", memory_size);
                continue;
            }

            free(ptr);
        }

        println!("Finish to allocate memory of size: {}", memory_size);

        let duration = start.elapsed().as_nanos();

        writeln!(file, "{},{}", memory_size, duration).expect("Failed to write to file");
    }
}
