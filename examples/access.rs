// List 4-2
#![feature(convert)]
extern crate apue;

use apue::unistd::*;
use apue::fcntl::*;
use std::ffi::CString;
use std::str;
use std::env;

use apue::consts::*;


fn main() {
    let args: Vec<CString> = env::args_os().map(|s| s.to_cstring().unwrap()).collect();
    if args.len() != 2 {
        panic!("usage: {} <pathname>", str::from_utf8(args[0].as_bytes()).unwrap());
    }
    unsafe {
        if access(args[1].as_ptr(), R_OK) < 0 {
            panic!("access error for {}", str::from_utf8(args[1].as_bytes()).unwrap())
        } else {
            println!("read access OK");
        }

        if open(args[1].as_ptr(), O_RDONLY, 0) < 0 {
            panic!("open error for {}", str::from_utf8(args[1].as_bytes()).unwrap());
        } else {
            println!("open for reading OK");
        }
    }
}
