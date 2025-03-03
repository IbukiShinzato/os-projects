use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn invalid_arguments() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "Usage: cargo run --release <source_file> <buckup_file1> <buckup_file2> <buckup_file3>"
        ),
    )
}

fn get_sha256(file_path: &str) -> io::Result<String> {
    let output = Command::new("sha256sum").arg(file_path).output()?;
    let hash_output = String::from_utf8_lossy(&output.stdout);
    let hash = hash_output.split_whitespace().next().ok_or(io::Error::new(
        io::ErrorKind::Other,
        "Failed to extract hash",
    ))?;

    Ok(hash.to_string())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        return Err(invalid_arguments());
    }

    let source_file = &args[1];
    let backup_files = &args[2..];

    let source_hash = get_sha256(source_file)?;

    println!("Source File: {}\nSHA256: {}\n", source_file, source_hash);

    let mut valid_count = 0;

    for backup_file in backup_files {
        let backup_hash = get_sha256(&backup_file)?;

        if source_hash == backup_hash {
            println!("✅ {} is VALID!", backup_file);
            valid_count += 1;
        } else {
            println!("❌ {} is NOT valid!", backup_file);
        }
    }

    println!(
        "\nSummary: {}/{} backups are valid.",
        valid_count,
        backup_files.len()
    );

    let mut log_file = fs::File::create("logs/backup_check.log")?;
    writeln!(log_file, "Source: {}\nHash: {}\n", source_file, source_hash)?;
    for backup_file in backup_files {
        let status = if get_sha256(backup_file)? == source_hash {
            "VALID"
        } else {
            "NOT VALID"
        };
        writeln!(log_file, "{}: {}", backup_file, status)?;
    }

    Ok(())
}
