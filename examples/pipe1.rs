// List 15-1
extern crate apue;
extern crate libc;

use apue::unistd::*;
use apue::consts::*;
use apue::MAXLINE;
use apue::string::strlen;
use std::ffi::CString;
use libc::{
    c_int,
    c_void,
    c_char,
    size_t
};

fn main() {
    let mut line = [0 as c_char; MAXLINE];

    unsafe {
        let mut fd = [0 as c_int; 2];

        if pipe(&mut fd[0]) < 0 {
            panic!("pipe error");
        }
        // println!("got fds => {:?}", fd);
        let pid = fork();
        if pid < 0 {
            panic!("fork error");
        } else if pid > 0 {     // parent
            close(fd[0]);
            let msg = CString::new("hello world\n").unwrap().as_ptr();
            write(fd[1], msg as *const c_void, strlen(msg));
        } else {                // child
            close(fd[1]);
            let n = read(fd[0], &mut line[0] as *mut c_char as *mut c_void, MAXLINE as size_t);
            write(STDOUT_FILENO, &line[0] as *const c_char as *const c_void, n as size_t);
        }
    }
}
