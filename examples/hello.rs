extern crate apue;
extern crate libc;

use libc::{
    getpid
};


fn main() {
    unsafe {
        println!("hello world from process ID {}", getpid())
    }
}
