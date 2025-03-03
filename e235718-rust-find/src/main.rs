use clap::{Arg, Command};
use std::env;
use std::process::{Command as ProcessCommand, Stdio};

fn main() {
    if let Some(user) = env::var_os("USER") {
        println!("Executing as user: {:?}", user);
    } else {
        println!("Unable to determine user");
    }

    let matches = Command::new("Find Command Wrapper")
        .version("1.0")
        .author("Ibuki Shinzato <e235718@ie.u-ryukyu.ac.jp>")
        .about("Searches for files in a directory with a given pattern")
        .arg(
            Arg::new("directory")
                .help("Directory to search")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("pattern")
                .short('f')
                .long("file")
                .required(true)
                .help("Pattern to match files (e.g., *.c)"),
        )
        .get_matches();

    let directory = matches
        .get_one::<String>("directory")
        .expect("Directory is required");
    let pattern = matches
        .get_one::<String>("pattern")
        .expect("Pattern is required");

    let output = ProcessCommand::new("find")
        .arg(directory)
        .arg("-type")
        .arg("f")
        .arg("-name")
        .arg(pattern)
        .arg("-exec")
        .arg("wc")
        .arg("{}")
        .arg("+")
        .stderr(Stdio::null())
        .output()
        .expect("Failed to execute find command");

    let result = String::from_utf8_lossy(&output.stdout);
    if result.is_empty() {
        println!("No files found matching the pattern.");
    } else {
        println!("Find command output:\n{}", result);
    }
}

