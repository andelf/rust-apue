// List 1-5
extern crate apue;
extern crate libc;

use std::ptr;
use std::io::prelude::*;
use std::io;
use std::process::exit;
use std::ffi::CStr;
use std::str;
use libc::{
    strlen,
    fgets,
    fork,
    // execlp,
    waitpid,
    c_char,
    c_int,
};

use apue::{
    MAXLINE,
    errno
};
use apue::stdio::stdin;

extern {
    // this is a var arg func in C
    // int execlp(const char *file, const char *arg0, ... /*, (char *)0 */);
    fn execlp(file: *const c_char, arg0: *const c_char, _: *const c_char);
}

fn main() {
    let mut buf: [c_char; MAXLINE] = [0; MAXLINE];
    print!("% ");
    io::stdout().flush().unwrap();
    unsafe {
        loop {
            if fgets(&mut buf[0], MAXLINE as c_int, stdin).is_null() {
                break;
            }

            let end_char_pos = strlen(&buf[0]) as usize - 1;
            if buf[end_char_pos] == '\n' as c_char {
                buf[end_char_pos] = 0;
            }

            let mut pid = fork();
            if pid < 0 {
                panic!("fork error");
            } else if pid == 0 { // child
                execlp(&buf[0], &buf[0], ptr::null());
                if errno() != 0 {
                    println!("couldn't execute: {}", str::from_utf8(CStr::from_ptr(&buf[0]).to_bytes()).unwrap());
                    exit(127);
                }
            }
            // parent
            let status = 0;
            pid = waitpid(pid, &status, 0);
            if pid < 0 {
                panic!("waitpid error");
            }

            print!("% ");
            io::stdout().flush().unwrap();
        }
    }
}
