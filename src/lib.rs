#![feature(plugin)]
#![plugin(bindgen_plugin)]


extern crate libc;


pub use libc::consts::os::posix88::*;

pub const MAXLINE: usize = 4096;


pub const FILE_MODE: libc::mode_t = (S_IRUSR | S_IWUSR | S_IRGRP | S_IROTH);

// pub use libc::funcs::posix88::unistd::*;



// errno
extern {
    #[cfg_attr(any(target_os = "macos",
                   target_os = "ios",
                   target_os = "freebsd"),
               link_name = "__error")]
    pub fn errnop() -> *mut libc::c_int;
}
pub fn errno() -> libc::c_int {
    unsafe {
        *errnop()
    }
}



pub mod fcntl {
    pub use libc::funcs::posix88::fcntl::{
        open,
        creat,
        fcntl
    };
}


pub mod unistd {
    pub use libc::funcs::posix88::unistd::*;
    pub use libc::consts::os::posix88::{
        STDIN_FILENO,
        STDOUT_FILENO,
        STDERR_FILENO,
        F_OK,
        R_OK,
        W_OK,
        X_OK
    };
}

#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub mod dirent {
    // the functions in this mod maybe linked with wrong version
    mod ffi {
        bindgen!("/usr/include/dirent.h", match="dirent.h");
    }

    pub use self::ffi::{
        DIR, Struct_dirent, closedir,
    };
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
}


pub mod stdio {
    use libc::{
        FILE,
        c_int,
        fgetc,
        fputc
    };

    pub use libc::{
        SEEK_SET,
        SEEK_CUR,
        SEEK_END
    };

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

#[allow(non_snake_case, overflowing_literals)]
pub mod stat {
    pub use libc::funcs::posix88::stat_::*;
    pub use libc::types::os::arch::posix01::stat;
    pub use libc::funcs::posix01::stat_::lstat;
    pub use libc::types::os::arch::posix88::mode_t;
    pub use libc::consts::os::posix88::{
        S_IEXEC,
        S_IFBLK,
        S_IFCHR,
        S_IFDIR,
        S_IFIFO,
        S_IFLNK,
        S_IFMT,
        S_IFREG,
        S_IREAD,
        S_IRGRP,
        S_IROTH,
        S_IRUSR,
        S_IRWXG,
        S_IRWXO,
        S_IRWXU,
        S_IWGRP,
        S_IWOTH,
        S_IWRITE,
        S_IWUSR,
        S_IXGRP,
        S_IXOTH,
        S_IXUSR,
    };

    pub const S_IFSOCK: mode_t = 0140000;

    #[inline]
    pub fn S_ISBLK(m: mode_t) -> bool {
        (((m) & S_IFMT) == S_IFBLK)     /* block special */
    }

    #[inline]
    pub fn S_ISCHR(m: mode_t) -> bool {
        (((m) & S_IFMT) == S_IFCHR)     /* char special */
    }

    #[inline]
    pub fn S_ISDIR(m: mode_t) -> bool {
        (((m) & S_IFMT) == S_IFDIR)     /* directory */
    }

    #[inline]
    pub fn S_ISFIFO(m: mode_t) -> bool {
        (((m) & S_IFMT) == S_IFIFO)     /* fifo or socket */
    }

    #[inline]
    pub fn S_ISREG(m: mode_t) -> bool {
        (((m) & S_IFMT) == S_IFREG)     /* regular file */
    }

    #[inline]
    pub fn S_ISLNK(m: mode_t) -> bool {
        (((m) & S_IFMT) == S_IFLNK)     /* symbolic link */
    }

    #[inline]
    pub fn S_ISSOCK(m: mode_t) -> bool {
        (((m) & S_IFMT) == S_IFSOCK)    /* socket */
    }
}
