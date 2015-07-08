// List 1-2
extern crate apue;
extern crate libc;

use std::mem;
use libc::{
    c_char,
    size_t,
    read,
    write,
    STDIN_FILENO,
    STDOUT_FILENO
};

const BUFFSIZE: usize = 4096;


fn main() {

    let mut buf: [c_char; BUFFSIZE] = [0; BUFFSIZE];
    let mut n;
    unsafe {
        loop {
            n = read(STDIN_FILENO, mem::transmute(&mut buf), BUFFSIZE as size_t);
            if n > 0 {
                if write(STDOUT_FILENO, mem::transmute(&buf), n as size_t) != n {
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
