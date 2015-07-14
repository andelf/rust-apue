// List 4-5
extern crate apue;

use apue::unistd::*;
use apue::fcntl::*;
use std::ffi::CString;
use apue::consts::*;

fn main() {
    unsafe {
        if open(CString::new("tempfile").unwrap().as_ptr(), O_RDWR, 0) < 0 {
            panic!("open error");
        }
        if unlink(CString::new("tempfile").unwrap().as_ptr()) < 0 {
            panic!("unlink error");
        }
        println!("file unlinked");
        sleep(15);
        println!("done");
    }
}
