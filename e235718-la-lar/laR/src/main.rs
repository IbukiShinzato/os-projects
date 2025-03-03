mod ls;

use ls::la;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut path = ".";
    let mut max_depth = 10;
    if args.contains(&"-d".to_string()) {
        if PartialEq::eq(args[1].as_str(), "-d") {
            if args.len() == 3 {
                max_depth += args[2].parse::<usize>().unwrap();
            } else {
                eprintln!("Error: -d option requires a number.");
                return Ok(());
            }
        } else if PartialEq::eq(args[2].as_str(), "-d") {
            path = args[1].as_str();
            if args.len() == 4 {
                max_depth += args[3].parse::<usize>().unwrap();
            } else {
                eprintln!("Error: -d option requires a number.");
                return Ok(());
            }
        }
    } else if args.len() >= 2 {
        path = args[1].as_str();
    }

    // directoryのスタック
    let stack = &mut Vec::with_capacity(max_depth);
    let start = path.to_string();
    stack.push(start);

    la(stack, max_depth)?;
    Ok(())
}
