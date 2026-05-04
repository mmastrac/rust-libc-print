#![cfg_attr(nostartfiles, no_main)]
use libc_print::*;

#[cfg(all(nostartfiles))]
#[no_mangle]
fn _start() {
    libc_println!("Hello, {}!", "world");
}

#[cfg(not(nostartfiles))]
fn main() {
    libc_println!("Hello, {}!", "world");
}
