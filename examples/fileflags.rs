// List 3-4
#![feature(convert)]
extern crate apue;
extern crate libc;


// cargo run --example fileflags 2 2>>temp.foo

use std::env;
use std::ffi::CString;
use apue::fcntl::*;

use libc::{
    atoi,
    c_int,
    F_GETFL,
    O_RDONLY,
    O_WRONLY,
    O_RDWR,
    O_APPEND,
    O_NONBLOCK,
    O_SYNC,
//    O_FSYNC
};
// use libc::consts::os::posix01::O_ACCMODE;
// FIXME:
pub const O_ACCMODE : c_int = 3;

fn main() {
    let args: Vec<CString> = env::args_os().map(|s| s.to_cstring().unwrap()).collect();

    if args.len() != 2 {
        panic!("useage: {} <descriptor#>", env::args().next().unwrap())
    }

    unsafe {
        let val = fcntl(atoi(args[1].as_ptr()), F_GETFL, 0);
        if val < 0 {
            panic!("fcntl error for fd {}", atoi(args[1].as_ptr()));
        }

        match val & O_ACCMODE {
            O_RDONLY => print!("read only"),
            O_WRONLY => print!("write only"),
            O_RDWR   => print!("read write"),
            _        => panic!("unknown access mode")
        }

        if val & O_APPEND != 0 {
            print!(", append");
        }
        if val & O_NONBLOCK != 0 {
            print!(", nonblocking");
        }

        if val & O_SYNC != 0 {
            print!(", synchronous writes");
        }

        // if val & O_FSYNC {
        //     print!(", synchronous writes");
        // }

        println!("");
    }
}
