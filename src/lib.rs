//! Implements `println!` and `eprintln!` on top of the `libc `crate without requiring
//! the use of an allocator.
//!
//! Allows you to use these macros in a #![no_std] context, or in a situation where the
//! traditional Rust streams might not be available (ie: at process shutdown time).
//!
//! ## Usage
//!
//! Exactly as you'd use `println!` or `eprintln!`.

#![no_std]
#![allow(dead_code)]

// These constants are used by the macros but we don't want to expose
// them to library users.
#[doc(hidden)]
pub const __LIBC_NEWLINE: &str = "\n";
#[doc(hidden)]
pub const __LIBC_STDOUT: u32 = 1;
#[doc(hidden)]
pub const __LIBC_STDERR: u32 = 2;

#[doc(hidden)]
pub struct LibCWriter(u32);

impl core::fmt::Write for LibCWriter {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __libc_println(self.0, s)
    }
}

impl LibCWriter {
    #[inline]
    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    #[inline]
    pub fn write_str(&mut self, s: &str) -> core::fmt::Result {
        core::fmt::Write::write_str(self, s)
    }
}

#[cfg(not(windows))]
#[doc(hidden)]
pub fn __libc_println(handle: u32, msg: &str) -> core::fmt::Result {
    unsafe {
        libc::write(
            handle as i32,
            msg.as_ptr() as *const core::ffi::c_void,
            msg.len() as libc::size_t,
        );
        Ok(())
    }
}

#[cfg(windows)]
#[doc(hidden)]
pub fn __libc_println(handle: u32, msg: &str) -> core::fmt::Result {
    unsafe {
        libc::write(
            handle as i32,
            msg.as_ptr() as *const core::ffi::c_void,
            msg.len() as u32,
        );
        Ok(())
    }
}

/// Macro for printing to the standard output, with a newline.
///
/// Does not panic on failure to write - instead silently ignores errors.
///
/// See [`println!`](https://doc.rust-lang.org/std/macro.println.html) for
/// full documentation.
///
/// You may wish to `use libc_print::std_name::*` to use a replacement
/// `println!` macro instead of this longer name.
#[macro_export]
macro_rules! libc_println {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::LibCWriter($crate::__LIBC_STDOUT);
            stm.write_fmt(format_args!($($arg)*));
            stm.write_str($crate::__LIBC_NEWLINE);
        }
    };
}

/// Macro for printing to the standard output.
///
/// Does not panic on failure to write - instead silently ignores errors.
///
/// See [`print!`](https://doc.rust-lang.org/std/macro.print.html) for
/// full documentation.
///
/// You may wish to `use libc_print::std_name::*` to use a replacement
/// `print!` macro instead of this longer name.
#[macro_export]
macro_rules! libc_print {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::LibCWriter($crate::__LIBC_STDOUT);
            stm.write_fmt(format_args!($($arg)*));
        }
    };
}

/// Macro for printing to the standard error, with a newline.
///
/// Does not panic on failure to write - instead silently ignores errors.
///
/// See [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html) for
/// full documentation.
///
/// You may wish to `use libc_print::std_name::*` to use a replacement
/// `eprintln!` macro instead of this longer name.
#[macro_export]
macro_rules! libc_eprintln {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::LibCWriter($crate::__LIBC_STDERR);
            stm.write_fmt(format_args!($($arg)*));
            stm.write_str($crate::__LIBC_NEWLINE);
        }
    };
}

/// Macro for printing to the standard error.
///
/// Does not panic on failure to write - instead silently ignores errors.
///
/// See [`eprint!`](https://doc.rust-lang.org/std/macro.eprint.html) for
/// full documentation.
///
/// You may wish to `use libc_print::std_name::*` to use a replacement
/// `eprint!` macro instead of this longer name.
#[macro_export]
macro_rules! libc_eprint {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::LibCWriter($crate::__LIBC_STDERR);
            stm.write_fmt(format_args!($($arg)*));
        }
    };
}

/// This package contains the `libc_print` macros, but using the stdlib names
/// such as `println!`, `print!`, etc.
#[macro_use]
pub mod std_name {
    /// Macro for printing to the standard output.
    ///
    /// Does not panic on failure to write - instead silently ignores errors.
    ///
    /// See [`print!`](https://doc.rust-lang.org/std/macro.print.html) for
    /// full documentation.
    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => {
            $crate::libc_print!($($arg)*);
        };
    }

    /// Macro for printing to the standard error, with a newline.
    ///
    /// Does not panic on failure to write - instead silently ignores errors.
    ///
    /// See [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html) for
    /// full documentation.
    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {
            $crate::libc_println!($($arg)*);
        };
    }

    /// Macro for printing to the standard error.
    ///
    /// Does not panic on failure to write - instead silently ignores errors.
    ///
    /// See [`eprint!`](https://doc.rust-lang.org/std/macro.eprint.html) for
    /// full documentation.
    #[macro_export]
    macro_rules! eprint {
        ($($arg:tt)*) => {
            $crate::libc_eprint!($($arg)*);
        };
    }

    /// Macro for printing to the standard error, with a newline.
    ///
    /// Does not panic on failure to write - instead silently ignores errors.
    ///
    /// See [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html) for
    /// full documentation.
    #[macro_export]
    macro_rules! eprintln {
        ($($arg:tt)*) => {
            $crate::libc_eprintln!($($arg)*);
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stdout() {
        super::libc_println!("stdout fd = {}", super::__LIBC_STDOUT);
    }

    #[test]
    fn test_stderr() {
        super::libc_eprintln!("stderr fd = {}", super::__LIBC_STDERR);
    }
}

#[cfg(test)]
mod tests_std_name {
    #[test]
    fn test_stdout() {
        println!("stdout fd = {}", super::__LIBC_STDOUT);
    }

    #[test]
    fn test_stderr() {
        eprintln!("stderr fd = {}", super::__LIBC_STDERR);
    }
}
