use core::panic;
// testing dynamic library from Rust
// which also has Serde as dependency
// TODO:: Need to find out how to also compile Serde as a dynamic lib
use dynotest::{add, MyType};
use std::ffi::{c_int, c_uchar, c_uint, c_ulong, CStr, CString};

use libc::c_char;

#[link(name = "z")]
extern "C" {
    fn compress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;
    fn compressBound(source_len: c_ulong) -> c_ulong;
    fn uncompress(
        dest: *mut u8,
        dest_len: *mut c_ulong,
        source: *const u8,
        source_len: c_ulong,
    ) -> c_int;
    fn hello_world() -> *const c_char;
    fn print_hello();
}

pub fn zlib_compress(source: &[u8]) -> Vec<u8> {
    unsafe {
        let source_len = source.len() as c_ulong;
        let mut dest_len = compressBound(source_len);
        let mut dest: Vec<u8> = Vec::with_capacity(dest_len as usize);
        compress(
            dest.as_mut_ptr(),
            &mut dest_len,
            source.as_ptr(),
            source_len,
        );
        dest.set_len(dest_len as usize);
        dest
    }
}
pub fn zlib_uncompress(source: &[u8], max_dest_len: usize) -> Vec<u8> {
    unsafe {
        let source_len = source.len() as c_ulong;
        let mut dest_len = max_dest_len as c_ulong;
        let mut dest: Vec<u8> = Vec::with_capacity(max_dest_len);
        uncompress(
            dest.as_mut_ptr(),
            &mut dest_len,
            source.as_ptr(),
            source_len,
        );
        dest.set_len(dest_len as usize);
        dest
    }
}

fn call_hello_world() -> &'static str {
    unsafe {
        print_hello();
        CStr::from_ptr(hello_world())
            .to_str()
            .expect("String conversion failed!")
    }
}

#[repr(C)]
struct GzFileState {
    have: c_uint,
    next: *mut c_uchar,
    pos: i64,
}

pub(crate) type GzFile = *mut GzFileState;

#[allow(unused)]
#[link_name = "z"]
extern "C" {
    fn gzopen(path: *const c_char, mode: *const c_char) -> GzFile;
    fn gzread(file: GzFile, buf: *mut c_uchar, len: c_uint) -> c_int;
    fn gzclose(file: GzFile) -> c_int;
    fn gzeof(file: GzFile) -> c_int;
}

pub fn read_gz_file(name: &str) -> String {
    let mut buffer = [0u8; 0x1000];
    let mut contents = String::new();
    unsafe {
        let c_name = CString::new(name).expect("CString failed");
        let c_mode = CString::new("r").expect("CString failed");
        let file = gzopen(c_name.as_ptr(), c_mode.as_ptr());
        if file.is_null() {
            panic!("Couldn't read file: {}", std::io::Error::last_os_error());
        }
        while gzeof(file) == 0 {
            let bytes_read = gzread(file, buffer.as_mut_ptr(), (buffer.len() - 1) as c_uint);
            let s = std::str::from_utf8(&buffer[..(bytes_read as usize)]).unwrap();
            contents.push_str(s);
        }
        gzclose(file);
    }
    contents
}

fn main() {
    let hello_world = "hello_world".as_bytes();
    let hello_world_compressed = zlib_compress(hello_world);
    println!(
        "hello_world_compressed: {:#?},\ncompressed len: {}, hello_world len:{}",
        hello_world_compressed,
        hello_world_compressed.len(),
        hello_world.len()
    );

    let hello_world_uncompressed = zlib_uncompress(&hello_world_compressed, 100);
    assert_eq!(hello_world, hello_world_uncompressed);
    println!(
        "hello_world_uncompressed : {}",
        String::from_utf8(hello_world_uncompressed).expect("Invalid character")
    );
    println!("call c_program hello world: {}", call_hello_world());
    println!("contents of main.rs: {:#?}", read_gz_file("src/main.rs.gz"));

    println!("add dynamic lib {}", add(5, 6));

    let my_type = MyType {
        value: 5,
        name: "Salitos".into(),
    };
    println!("my types from dyno : {:#?}", my_type);
}
