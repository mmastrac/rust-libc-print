#![cfg_attr(nostartfiles, no_main)]
use libc_print::*;

#[cfg(all(nostartfiles))]
#[no_mangle]
fn __start() {
    libc_println!("Hello, {}!", "world");
}

#[cfg(not(nostartfiles))]
fn main() {
    libc_println!("Hello, {}!", "world");
}
