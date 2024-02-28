use std::ffi::{c_int, c_uchar, c_uint, c_ulong, CStr};

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
        CStr::from_ptr(hello_world())
            .to_str()
            .expect("String conversion failed!")
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
struct gzFile_s {
    have: c_uint,
    next: *mut c_uchar,
    pos: i64,
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
}
