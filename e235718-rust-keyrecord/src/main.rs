// keyrecord api in rust
use crossterm::{
    event::{self, KeyCode, KeyEvent},
    terminal,
};
use std::fs::File;
use std::io::{self, Write};

pub fn record_keys() -> io::Result<String> {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");

    let mut file =
        File::create("keyboard_input.txt").expect("Failed to create the file for keyboard input");

    print!("Type something and press Enter (Press 'q' to quit): ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();

    loop {
        if event::poll(std::time::Duration::from_millis(100)).expect("Failed to poll for events") {
            if let event::Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = event::read().expect("Failed to read keyboard input")
            {
                if code == KeyCode::Char('q') {
                    writeln!(file, "\nkeyboard_input: {}", input)
                        .expect("Failed to write to the file");
                    break;
                }

                print!("{}", code);
                io::stdout().flush().expect("Failed to flush stdout");

                writeln!(
                    file,
                    "Key: {:?} | Modifiers: {:?} | Kind: {:?} | State: {:?} ",
                    code, modifiers, kind, state
                )
                .expect("Failed to write key event to the file");

                input += &format!("{}", code);
            }
        }
    }

    terminal::disable_raw_mode().expect("Failed to disable raw mode");
    Ok(input)
}

// for gitlab-ci
pub fn record_keys_with_input(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok(input.to_string())
}

pub fn main() {
    match record_keys() {
        Ok(input) => println!("\nCaptured Input: {}", input),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
include!("./record_keys_test.rs");
