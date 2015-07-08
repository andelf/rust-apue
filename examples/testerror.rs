// List 1-6
extern crate apue;
extern crate libc;

use std::io::prelude::*;
use std::io;
use std::str;
use std::env;
use std::ffi::{CStr, CString};
use libc::{
    strerror,
    perror
};
use apue::{
    errno,
};


fn main() {
    unsafe {
        write!(io::stderr(), "EACCESS: {}\n",
               str::from_utf8(CStr::from_ptr(strerror(errno())).to_bytes()).unwrap()).unwrap();
        perror(CString::new(env::args().next().unwrap()).unwrap().as_ptr())
    }
}
