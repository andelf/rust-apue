// List 3-2
extern crate apue;
extern crate libc;

use std::ffi::CString;

use apue::fcntl::*;
use apue::unistd::*;
use apue::stdio::*;
use apue::FILE_MODE;

use libc::c_void;


fn main() {
    let buf1 = CString::new("abcdefghij").unwrap();
    let buf2 = CString::new("ABCDEFGHIJ").unwrap();

    unsafe {
        let fd = creat(CString::new("file.hole").unwrap().as_ptr(), FILE_MODE);
        if fd < 0 {
            panic!("creat error");
        }

        if write(fd, buf1.as_ptr() as *const c_void, 10) != 10 {
            panic!("buf1 write error");
        }
        // offset now = 10

        if lseek(fd, 16384, SEEK_SET) == -1 {
            panic!("lseek error");
        }
        // offset now = 16384

        if write(fd, buf2.as_ptr() as *const c_void, 10) != 10 {
            panic!("buf2 write error");
        }
        // offset now = 16394
    }
}
