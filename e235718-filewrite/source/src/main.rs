use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::time::Instant;

fn read_data(buffer_size: usize, file_size_kb: usize) -> std::io::Result<u64> {
    let file = File::open(format!("output_{}KB.dat", file_size_kb))?;
    let mut reader = BufReader::with_capacity(buffer_size, file);

    let mut buffer = match buffer_size {
        0 => vec![0u8; 1],
        _ => vec![0u8; buffer_size],
    };

    let start = Instant::now();
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        println!("Read {} bytes", bytes_read);
    }
    let duration_ns = start.elapsed().as_nanos();
    Ok(duration_ns as u64)
}

fn write_data(buffer_size: usize, file_size_kb: usize) -> std::io::Result<()> {
    let file_name = format!("output_{}KB.dat", file_size_kb);
    let file = File::create(file_name)?;

    let data_size = 1024;
    let data = vec![0u8; data_size];
    let iterations = file_size_kb * 1024 / data_size;

    let mut writer = BufWriter::with_capacity(buffer_size, file);
    let mut buffer = Vec::with_capacity(buffer_size);

    for _ in 0..iterations {
        buffer.extend_from_slice(&data);
        if buffer.len() >= buffer_size {
            writer.write_all(&buffer)?;
            buffer.clear();
        }
    }

    if !buffer.is_empty() {
        writer.write_all(&buffer)?;
    }
    writer.flush()?;

    Ok(())

    // let data_size = match buffer_size {
    //     0 => 128,
    //     2 => 512,
    //     4 => 2048,
    //     8 => 4096,
    //     16 => 8192,
    //     32 => 16384,
    //     64 => 32768,
    //     128 => 65536,
    //     256 => 131072,
    //     512 => 262144,
    //     1024 => 524288,
    //     2048 => 1048576,
    //     4096 => 2097152,
    //     8192 => 4194304,
    //     16384 => 8388608,
    //     32768 => 16777216,
    //     _ => 1024,
    // };

    // let start = Instant::now();

    // if buffer_size == 0 {
    //     for _ in 0..iterations {
    //         file.write_all(&data)?;
    //     }
    // } else {
    //     let mut writer = BufWriter::with_capacity(buffer_size, file);
    //     let mut buffer = Vec::with_capacity(buffer_size);

    //     for _ in 0..iterations {
    //         if buffer.len() >= buffer_size {
    //             writer.write_all(&buffer)?;
    //             buffer.clear();
    //         }
    //     }

    //     if !buffer.is_empty() {
    //         writer.write_all(&buffer)?;
    //     }
    //     writer.flush()?;
    // }

    // let duration_ns = start.elapsed().as_nanos();
    // Ok(duration_ns as u64)
}

fn merge_csv_files(buffer_sizes: &[usize]) -> std::io::Result<()> {
    // let mut merged_file = File::create("./data/write_times.csv")?;
    let mut merged_file = File::create("./data/read_macos/read_times.csv")?;
    writeln!(merged_file, "Buffer Size (Bytes),File Size (KB),Time (ns)")?;

    let mut num = 1;
    for &buffer_size in buffer_sizes {
        let file_name = if num < 10 {
            format!("./data/read_macos/0{}_data_buffer_{}.csv", num, buffer_size)
        } else {
            format!("./data/read_macos/{}_data_buffer_{}.csv", num, buffer_size)
        };

        let content = fs::read_to_string(&file_name)?;
        num += 1;

        for (i, line) in content.lines().enumerate() {
            if i == 0 {
                continue;
            }
            let fields: Vec<&str> = line.split(",").collect();
            if let (Some(size_kb), Some(time_ns)) = (fields.get(0), fields.get(1)) {
                writeln!(merged_file, "{},{},{}", buffer_size, size_kb, time_ns)?;
            }
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let file_sizes_kb = vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];
    let buffer_sizes = vec![
        0, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768,
    ];

    for &file_size_kb in &file_sizes_kb {
        let _ = write_data(32768, file_size_kb);
    }

    let mut num = 1;
    for &buffer_size in &buffer_sizes {
        let csv_file_name = if num < 10 {
            format!("./data/read_macos/0{}_data_buffer_{}.csv", num, buffer_size)
        } else {
            format!("./data/read_macos/{}_data_buffer_{}.csv", num, buffer_size)
        };

        let mut csv_file = File::create(&csv_file_name)?;
        writeln!(csv_file, "File Size (KB),Time (ns)")?;
        num += 1;

        for &file_size_kb in &file_sizes_kb {
            // let time = write_data(buffer_size, file_size_kb)?;
            let time = read_data(buffer_size, file_size_kb)?;
            writeln!(csv_file, "{},{}", file_size_kb, time)?;
            println!(
                "Buffer Size: {}, File Size: {}, Time: {}",
                buffer_size, file_size_kb, time
            );
        }
    }

    merge_csv_files(&buffer_sizes)?;

    Ok(())
}
