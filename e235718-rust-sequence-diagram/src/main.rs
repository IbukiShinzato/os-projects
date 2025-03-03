use fork::{fork, Fork};
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::{env, process};

fn main() {
    let commands: Vec<String> = env::args().collect();
    let arg = commands.get(1).unwrap();

    // ";"で分割
    let args: Vec<&str> = arg.split(';').collect();

    for arg in args {
        match fork() {
            //　親プロセス
            Ok(Fork::Parent(ppid)) => {
                println!("\nParent {}", process::id());

                match waitpid(Some(Pid::from_raw(ppid)), None) {
                    Ok(WaitStatus::Exited(_, _status)) => {
                        println!("Child end");
                    }
                    _ => {
                        println!("Unexpected wait status");
                    }
                }
            }

            // 子プロセス
            Ok(Fork::Child) => {
                println!("Child {}", process::id());

                // 両サイドの空白を処理
                let trimmed_args = arg.trim_start().trim_end();

                // 正しい args
                let args: Vec<&str> = trimmed_args.split(" ").collect();
                let program = *args.get(0).unwrap();
                let iter_args = args.split_at(1).1;

                // execvp system call
                Command::new(program).args(iter_args).exec();
            }

            Err(_) => println!("Fork failed"),
        }
    }
}
