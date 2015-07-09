// List 3-1
extern crate apue;

//use apue::fcntl::*;
use apue::unistd::*;
use apue::stdio::*;
use apue::consts::*;

fn main() {
    unsafe {
        if lseek(STDIN_FILENO, 0, SEEK_CUR) == -1 {
            println!("cannot seek");
        } else {
            println!("seek OK");
        }
    }
}
