// List 1-1
#![feature(convert)]

extern crate apue;
extern crate libc;


//use libc::types::common::posix88::*;
use std::ffi::{CString, CStr};
use std::str;
use apue::dirent::closedir;
use apue::{opendir, readdir};

use std::env;


fn main() {
    let args: Vec<CString> = env::args_os().map(|s| s.to_cstring().unwrap()).collect();

    if args.len() != 2 {
        panic!("useage: ls directory_name");
    }

    unsafe {
        let dp = opendir(args[1].as_ptr());
        if dp.is_null() {
            panic!("can't open {:?}", args[1]);
        }

        let mut dirp = readdir(dp);
        while !dirp.is_null() {
            println!("{}", str::from_utf8(CStr::from_ptr(&(*dirp).d_name[0]).to_bytes()).unwrap());
            dirp = readdir(dp);
        }
        closedir(dp);
    }
}
