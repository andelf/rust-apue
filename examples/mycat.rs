// List 1-2
// List 3-3
extern crate apue;
extern crate libc;

use apue::unistd::*;
use libc::{
    c_char,
    size_t,
    c_void
};

const BUFFSIZE: usize = 4096;


fn main() {

    let mut buf: [c_char; BUFFSIZE] = [0; BUFFSIZE];
    let mut n;
    unsafe {
        loop {
            n = read(STDIN_FILENO, &mut buf[0] as *mut c_char as *mut c_void, BUFFSIZE as size_t);
            if n > 0 {
                if write(STDOUT_FILENO, &buf[0] as *const c_char as *const c_void, n as size_t) != n {
                    panic!("write error");
                }
            } else {
                break;
            }
        }
    }

    if n < 0 {
        panic!("read error");
    }
}
