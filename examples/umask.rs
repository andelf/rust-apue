// List 4-3
#![feature(convert)]
extern crate apue;

use apue::unistd::*;
use apue::stat::*;
use apue::fcntl::*;
use std::ffi::CString;
use std::str;
use std::env;
use std::mem;
use apue::consts::*;
use apue::types::*;

const RWRWRW: mode_t = S_IRUSR|S_IWUSR|S_IRGRP|S_IWGRP|S_IROTH|S_IWOTH;



fn main() {
    unsafe {
        umask(0);
        if creat(CString::new("foo").unwrap().as_ptr(), RWRWRW) < 0 {
            panic!("creat error for foo");
        }

        umask(S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH);
        if creat(CString::new("bar").unwrap().as_ptr(), RWRWRW) < 0 {
            panic!("creat error for bar");
        }
    }
}
