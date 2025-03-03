use std::env::args;
use std::io;

mod copy;

fn invalid_arguments() -> io::Result<()> {
    eprintln!("Invalid argements.");
    eprintln!("Usage: cargo run -- <method> <source_file> <target_file>");
    return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Invalid arguments",
    ));
}

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    if args.len() != 4 {
        invalid_arguments()?;
    }

    let (method, source, target) = (&args[1], &args[2], &args[3]);

    match method.as_str() {
        "mmap_copy" => copy::mmap_copy_wrapper(source, target)?,
        "mmap_write" => copy::mmap_write(source, target)?,
        "read_write" => copy::read_write(source, target)?,
        _ => invalid_arguments()?,
    };
    Ok(())
}
