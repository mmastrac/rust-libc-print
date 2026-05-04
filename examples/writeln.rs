#![cfg_attr(nostartfiles, no_main)]
use libc_print::*;

#[cfg(all(nostartfiles, not(windows)))]
fn _start() {
    libc_println!("Hello, {}!", "world");
}

#[cfg(all(nostartfiles, windows))]
#[no_mangle]
fn mainCRTStartup() {
    libc_println!("Hello, {}!", "world");
}

#[cfg(not(nostartfiles))]
fn main() {
    libc_println!("Hello, {}!", "world");
}
