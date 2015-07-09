// List 4-1
#![feature(convert)]
extern crate apue;

use apue::stat::*;
use std::ffi::{CString, CStr};
use std::str;
use std::env;
use std::mem;


fn main() {
    let args: Vec<CString> = env::args_os().map(|s| s.to_cstring().unwrap()).collect();
    for arg in args[1..].iter() {
        print!("{:?}: ", arg);

        unsafe {
            let mut buf: stat = mem::zeroed();
            if lstat(arg.as_ptr(), &mut buf) < 0 {
                println!("lstat error");
                continue;
            }

            let description = if S_ISREG(buf.st_mode) {
                "regular"
            } else if S_ISDIR(buf.st_mode) {
                "directory"
            } else if S_ISCHR(buf.st_mode) {
                "character special"
            } else if S_ISBLK(buf.st_mode) {
                "block special"
            } else if S_ISFIFO(buf.st_mode) {
                "fifo"
            } else if S_ISLNK(buf.st_mode) {
                "symbolic link"
            } else if S_ISSOCK(buf.st_mode) {
                "socket"
            } else {
                "** unknown mode **"
            };

            println!("{}", description);
        }
    }
}
