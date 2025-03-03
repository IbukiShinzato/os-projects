use libc::{
    close, fstat, madvise, mmap, open, stat, MADV_SEQUENTIAL, MAP_FAILED, MAP_SHARED, O_RDONLY,
    PROT_READ,
};
use std::ffi::CString;
use std::fs::File;
use std::io::{self, Read, Write};
use std::{mem, ptr, slice};

extern "C" {
    fn mmap_copy(source_file: *const i8, target_file: *const i8);
}

pub fn mmap_copy_wrapper(source: &str, target: &str) -> io::Result<()> {
    let src_c = CString::new(source).unwrap();
    let dst_c = CString::new(target).unwrap();

    unsafe {
        mmap_copy(src_c.as_ptr(), dst_c.as_ptr());
    }

    Ok(())
}

pub fn mmap_write(source_file: &str, target_file: &str) -> io::Result<()> {
    let source = CString::new(source_file)?;

    let fd;

    fd = unsafe { open(source.as_ptr(), O_RDONLY) };
    if fd < 0 {
        eprintln!("Failed to open {:?}", source);
    }

    let mut sb: stat = unsafe { mem::zeroed() };

    let res = unsafe { fstat(fd, &mut sb) };
    if res == -1 {
        eprintln!("Error with fstat");
        unsafe { close(fd) };
    }

    let map_size = sb.st_size as usize;
    let map = unsafe { mmap(ptr::null_mut(), map_size, PROT_READ, MAP_SHARED, fd, 0) };

    if map == ptr::null_mut() {
        eprintln!("Error with mmap");
        unsafe { close(fd) };
    }

    if map == MAP_FAILED {
        eprintln!("Failed to mmap");
    }

    unsafe {
        madvise(map, map_size, MADV_SEQUENTIAL);
    }

    let mapped_data: &[u8] = unsafe { slice::from_raw_parts(map as *const u8, map_size) };

    let mut target = File::create(target_file).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to create {}: {}", target_file, e))
    })?;

    target.write_all(mapped_data)?;

    unsafe {
        close(fd);
    }

    Ok(())
}

pub fn read_write(source_file: &str, target_file: &str) -> io::Result<()> {
    let mut source = File::open(source_file)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to open {}: {}", source_file, e)))?;
    let mut target = File::create(target_file).map_err(|e| {
        io::Error::new(e.kind(), format!("Failed to create {}: {}", target_file, e))
    })?;

    let mut buffer = [0u8; 5 * 1024 * 1024]; // 5MB
    loop {
        let bytes_read = source.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        target.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}
