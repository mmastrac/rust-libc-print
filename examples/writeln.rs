#![cfg_attr(nostartfiles, no_main)]
use libc_print::*;

#[cfg(nostartfiles)]
fn _start() {
    libc_println!("Hello, {}!", "world");
}

#[cfg(not(nostartfiles))]
fn main() {
    libc_println!("Hello, {}!", "world");
}
