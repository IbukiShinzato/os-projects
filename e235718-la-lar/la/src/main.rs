use chrono::{Local, NaiveDateTime, TimeZone};
use nix::unistd::{Gid, Group, Uid, User};
use std::fs::metadata;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::{fs, io};

fn format_unix_timestamp(timestamp: u64) -> String {
    let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let datetime = Local.from_utc_datetime(&naive);
    datetime.format("%b %e %H:%M").to_string()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let path = if args.len() < 2 {
        "."
    } else {
        &args[args.len() - 1]
    };

    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    // "."と".."は追加済み
    // 相対パスで入れる必要がある
    entries.insert(0, std::path::PathBuf::from("."));
    entries.insert(1, std::path::PathBuf::from(".."));

    let mut vecfiles: Vec<Vec<String>> = Vec::new();

    let mut index = 0;
    let mut block_count = 0;

    for file in entries {
        let metadata = metadata(file.clone())?;

        // ブロック数
        block_count += metadata.blocks();

        // 権限
        let permissions = metadata.permissions();

        // println!("permissions: {:o}, {:?}", permissions.mode(), file);

        let strper = format!("{:o}", permissions.mode());
        let mut addper = String::from("");

        if metadata.is_dir() {
            addper += "d";
        } else if metadata.is_file() {
            addper += "-";
        } else {
            addper += "?";
        }

        let last_three = &strper[strper.len() - 3..];
        for num in last_three.chars() {
            match num {
                '0' => addper += "---",
                '1' => addper += "--x",
                '2' => addper += "-w-",
                '3' => addper += "-wx",
                '4' => addper += "r--",
                '5' => addper += "r-x",
                '6' => addper += "rw-",
                '7' => addper += "rwx",
                _ => (),
            }
        }

        // println!("{}", addper);
        vecfiles.push(Vec::new());
        vecfiles[index].push(addper);

        // リンク数
        let hard_link_count = metadata.nlink().to_string();
        vecfiles[index].push(hard_link_count);

        // usernameとgroupname
        let uid = metadata.uid();
        let gid = metadata.gid();
        let user_name = User::from_uid(Uid::from_raw(uid)).unwrap().unwrap().name;
        let group_name = Group::from_gid(Gid::from_raw(gid)).unwrap().unwrap().name;
        vecfiles[index].push(user_name);
        vecfiles[index].push(group_name);

        // ファイルサイズ
        vecfiles[index].push(metadata.size().to_string());

        // 更新日時
        vecfiles[index].push(format_unix_timestamp(metadata.mtime() as u64));

        // パス名
        if file == std::path::PathBuf::from(".") {
            vecfiles[index].push(".".to_string());
        } else if file == std::path::PathBuf::from("..") {
            vecfiles[index].push("..".to_string());
        } else {
            let path = format!("{:?}", file);
            let path = path.trim_matches('"').to_string();
            let filename = path.split("/").last().unwrap_or("");
            vecfiles[index].push(filename.to_string());
        }
        index += 1;
    }

    println!("total {}", block_count);
    for file in vecfiles {
        println!("{}", file.join(" "));
    }

    Ok(())
}
