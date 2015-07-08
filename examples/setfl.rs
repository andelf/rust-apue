// List 3-5
extern crate apue;
extern crate libc;

use apue::fcntl::*;

use libc::{
    c_int,
    F_GETFL,
    F_SETFL,
    // O_RDONLY,
    // O_WRONLY,
    // O_RDWR,
    // O_APPEND,
    O_NONBLOCK,
    O_SYNC,
};


unsafe fn set_fl(fd: c_int, flags: c_int) {
    let mut val = fcntl(fd, F_GETFL, 0);
    if val < 0 {
        panic!("fcntl F_GETFL error");
    }

    val |= flags;               // turn on flags

    if fcntl(fd, F_SETFL, val) < 0 {
        panic!("fcntl F_SETFL error");
    }
}

fn main() {
    unsafe {
        set_fl(0, O_SYNC | O_NONBLOCK);
    }
}
