use std::fs;
use std::io::Write;
use std::path::Path;

#[cfg(target_os = "macos")]
mod macos {
    use super::*;
    use nix::fcntl::OFlag;
    use nix::sys::stat::Mode;
    use nix::unistd::close;
    use std::ffi::CString;
    use std::ptr;

    pub fn watch_file(file_path: &str, log_file_path: &str, is_test: bool) {
        let c_file_path = CString::new(file_path).expect("Failed to convert path to CString");

        if !is_test && !Path::new(file_path).exists() {
            fs::File::create(file_path).expect("Failed to create test file");
        }

        let kq = unsafe { libc::kqueue() };
        if kq == -1 {
            panic!("Failed to create kqueue");
        }

        let fd = unsafe {
            libc::open(
                c_file_path.as_ptr(),
                OFlag::O_RDONLY.bits() as i32,
                Mode::empty().bits() as i32,
            )
        };

        if fd == -1 {
            eprintln!("Failed to open file: {}", std::io::Error::last_os_error());
            panic!("Failed to open file");
        }

        let mut change = libc::kevent {
            ident: fd as libc::uintptr_t,
            filter: libc::EVFILT_VNODE,
            flags: libc::EV_ADD | libc::EV_ENABLE,
            fflags: libc::NOTE_WRITE | libc::NOTE_DELETE | libc::NOTE_RENAME,
            data: 0,
            udata: ptr::null_mut(),
        };

        let mut events = [libc::kevent {
            ident: 0,
            filter: 0,
            flags: 0,
            fflags: 0,
            data: 0,
            udata: ptr::null_mut(),
        }];

        let mut log_file = fs::File::create(log_file_path).expect("Failed to create log file");

        loop {
            let nev =
                unsafe { libc::kevent(kq, &mut change, 1, events.as_mut_ptr(), 1, ptr::null()) };
            if nev == -1 {
                eprintln!("kevent error: {}", std::io::Error::last_os_error());
                break;
            } else if nev > 0 {
                for event in events.iter().take(nev as usize) {
                    if event.filter == libc::EVFILT_VNODE as i16 {
                        match event.fflags {
                            libc::NOTE_WRITE => {
                                writeln!(log_file, "File modified: {:?}", file_path)
                                    .expect("Failed to write to log file");
                                println!("File modified: {:?}", file_path);
                            }
                            libc::NOTE_DELETE => {
                                writeln!(log_file, "File deleted: {:?}", file_path)
                                    .expect("Failed to write to log file");
                                println!("File deleted: {:?}", file_path);
                            }
                            libc::NOTE_RENAME => {
                                writeln!(log_file, "File renamed: {:?}", file_path)
                                    .expect("Failed to write to log file");
                                println!("File renamed: {:?}", file_path);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        close(fd).expect("Failed to close file");
        close(kq).expect("Failed to close kqueue");
    }
}

#[cfg(target_os = "linux")]
mod linux {
    use std::fs;
    use std::io::{BufRead, BufReader, Write};
    use std::os::unix::io::AsRawFd;

    pub fn watch_file(file_path: &str, log_file_path: &str, is_test: bool) {
        use inotify::{Inotify, WatchMask};

        if !is_test && !Path::new(file_path).exists() {
            fs::File::create(file_path).expect("Failed to create test file");
        }

        let mut inotify = Inotify::init().expect("Failed to initialize inotify");

        inotify
            .add_watch(
                file_path,
                WatchMask::MODIFY | WatchMask::DELETE_SELF | WatchMask::MOVE_SELF,
            )
            .expect("Failed to add inotify watch");

        let mut buffer = [0; 1024];
        let mut log_file = fs::File::create(log_file_path).expect("Failed to create log file");

        loop {
            let events = inotify
                .read_events_blocking(&mut buffer)
                .expect("Failed to read inotify events");

            for event in events {
                if event.mask.contains(WatchMask::MODIFY) {
                    writeln!(log_file, "File modified: {}", file_path)
                        .expect("Failed to write to log file");
                    println!("File modified: {}", file_path);
                }
                if event.mask.contains(WatchMask::DELETE_SELF) {
                    writeln!(log_file, "File deleted: {}", file_path)
                        .expect("Failed to write to log file");
                    println!("File deleted: {}", file_path);
                    break;
                }
                if event.mask.contains(WatchMask::MOVE_SELF) {
                    writeln!(log_file, "File renamed: {}", file_path)
                        .expect("Failed to write to log file");
                    println!("File renamed: {}", file_path);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_file() {
        let file_path = "test_file.txt";
        let log_file_path = "test_log.txt";

        // Create the file for testing
        fs::File::create(file_path).expect("Failed to create test file");

        // Run the watcher in a separate thread
        std::thread::spawn(move || {
            #[cfg(target_os = "macos")]
            macos::watch_file(file_path, log_file_path, true);

            #[cfg(target_os = "linux")]
            linux::watch_file(file_path, log_file_path, true);
        });

        // Modify the file to trigger events
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .expect("Failed to open test file");
        writeln!(file, "Test write").expect("Failed to write to test file");

        // Cleanup
        fs::remove_file(file_path).expect("Failed to remove test file");
        fs::remove_file(log_file_path).expect("Failed to remove log file");
    }
}

fn main() {
    let file_path = "test.txt";
    let log_file_path = "log/file_watcher.log";

    #[cfg(target_os = "macos")]
    macos::watch_file(file_path, log_file_path, false);

    #[cfg(target_os = "linux")]
    linux::watch_file(file_path, log_file_path, false);
}
 