extern crate apue;
extern crate libc;

use libc::{
    ferror,
    EOF
};

use apue::stdio::{
    stdin,
    stdout,
    getc,
    putc
};


fn main() {
    unsafe {
        loop {
            let c = getc(stdin);
            if c != EOF {
                if putc(c, stdout) == EOF {
                    panic!("output error");
                }
            } else {
                break;
            }
        }
        if ferror(stdin) != 0 {
            panic!("input error");
        }
    }
}
