#![cfg_attr(nostartfiles, no_main)]
use libc_print::*;

#[cfg(all(nostartfiles))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    libc_println!("Hello, {}!", "world");
    unsafe { libc::exit(0); }
}

#[cfg(not(nostartfiles))]
fn main() {
    libc_println!("Hello, {}!", "world");
}
