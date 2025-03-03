use std::env::args;

use passwords::PasswordGenerator;

fn main() {
    let mut args: Vec<String> = args().collect();

    // debug binary fileのindexを取得
    let remove_index = args
        .iter()
        .position(|x| *x == "target/debug/password_generation")
        .unwrap();

    // debug binary fileの削除
    args.remove(remove_index);

    // デフォルトの文字数
    let mut _wordlength = 16;

    // デフォルトの個数
    let mut _count = 10;

    let mut index = 0;
    for arg in &args {
        if arg == "-w" {
            match args.get(index + 1) {
                // usizeに変換可能か
                Some(next_arg) => match next_arg.parse::<usize>() {
                    Ok(value) => _wordlength = value,
                    Err(_) => panic!("No argument provided after '-w'"),
                },

                // index + 1にアクセス可能か
                None => panic!("Failed to parse value after '-w' as usize"),
            }
        }

        if arg == "-c" {
            match args.get(index + 1) {
                // usizeに変換可能か
                Some(next_arg) => match next_arg.parse::<usize>() {
                    Ok(value) => _count = value,
                    Err(_) => panic!("No argument provided after '-c'"),
                },

                // index + 1にアクセス可能か
                None => panic!("Failed to parse value after '-c' as usize"),
            }
        }

        index += 1;
    }

    let password_generator = PasswordGenerator {
        length: _wordlength,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: true,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };

    let passwords = password_generator.generate(_count).unwrap();
    println!("{}", passwords.join("\n"));
}
