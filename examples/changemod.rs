// List 4-4
extern crate apue;

use apue::stat::*;
use std::ffi::CString;
use std::mem;
use apue::consts::*;

fn main() {
    unsafe {
        let mut statbuf = mem::zeroed();

        if stat(CString::new("foo").unwrap().as_ptr(), &mut statbuf) < 0 {
            panic!("stat error for foo");
        }

        if chmod(CString::new("foo").unwrap().as_ptr(),
                 statbuf.st_mode & !S_IXGRP) < 0{
            panic!("chmod error for foo");
        }

        // set absolute mode to "rw-r--r--"
        if chmod(CString::new("bar").unwrap().as_ptr(),
                 S_IRUSR | S_IWUSR | S_IWGRP | S_IROTH) < 0 {
            panic!("chmod error for bar");
        }
    }
}
