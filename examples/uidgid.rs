// List 1-7
extern crate libc;

use libc::{
    getuid,
    getgid
};


fn main() {
    unsafe {
        println!("uid = {}, gid = {}", getuid(), getgid());
    }
}
