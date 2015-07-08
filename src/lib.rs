#![feature(plugin)]
#![plugin(bindgen_plugin)]


extern crate libc;

pub const MAXLINE: usize = 4096;

pub use libc::consts::os::posix88::{
    STDIN_FILENO,
    STDOUT_FILENO,
    STDERR_FILENO
};

pub use libc::funcs::posix88::unistd::*;



// errno
extern {
    #[cfg_attr(any(target_os = "macos",
                   target_os = "ios",
                   target_os = "freebsd"),
               link_name = "__error")]
    pub fn errno() -> *mut libc::c_int;
}





#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub mod dirent {
    // the functions in this mod maybe linked with wrong version
    bindgen!("/usr/include/dirent.h", match="dirent.h");
}

use dirent::{DIR, Struct_dirent};
use libc::c_char;
#[cfg(target_os = "macos")]
extern {
    #[link_name="opendir$INODE64"]
    pub fn opendir(arg1: *const c_char) -> *mut DIR;
    #[link_name="readdir$INODE64"]
    pub fn readdir(arg1: *mut DIR) -> *mut Struct_dirent;
}

#[cfg(not(target_os = "macos"))]
extern {
    #[link_name="opendir"]
    pub fn opendir(arg1: *const c_char) -> *mut DIR;
    #[link_name="readdir"]
    pub fn readdir(arg1: *mut DIR) -> *mut Struct_dirent;
}

pub mod stdio {
    use libc::*;

    extern {
        #[link_name="__stdinp"]
        pub static stdin: *mut FILE;

        #[link_name="__stdoutp"]
        pub static stdout: *mut FILE;
    }

    #[inline]
    pub unsafe fn getc(stream: *mut FILE) -> c_int {
        fgetc(stream)
    }

    #[inline]
    pub unsafe fn putc(c: c_int, stream: *mut FILE) -> c_int {
        fputc(c, stream)
    }
}
