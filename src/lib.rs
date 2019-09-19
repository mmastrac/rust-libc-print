//! Implements `println!` and `eprintln!` on top of the `libc `crate without requiring
//! the use of an allocator.
//!
//! Allows you to use these macros in a #![no_std] context, or in a situation where the
//! traditional Rust streams might not be available (ie: at process shutdown time).
//!
//! [`libc_writeln`] and [`libc_ewriteln`] are provided for cases where you may not wish
//! to pull in the overhead of the formatter code and simply wish to print C-style strings.
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
pub const __LIBC_STDOUT: i32 = 1;
#[doc(hidden)]
pub const __LIBC_STDERR: i32 = 2;

#[doc(hidden)]
pub struct __LibCWriter(i32);

impl core::fmt::Write for __LibCWriter {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __libc_println(self.0, s)
    }
}

impl __LibCWriter {
    #[inline]
    pub fn new(handle: i32) -> __LibCWriter {
        __LibCWriter(handle)
    }

    #[inline]
    pub fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    #[inline]
    pub fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __libc_println(self.0, s)
    }

    #[inline]
    pub fn write_nl(&mut self) -> core::fmt::Result {
        __libc_println(self.0, __LIBC_NEWLINE)
    }
}

#[cfg(not(windows))]
#[doc(hidden)]
#[inline]
pub fn __libc_println(handle: i32, msg: &str) -> core::fmt::Result {
    unsafe {
        libc::write(
            handle,
            msg.as_ptr() as *const core::ffi::c_void,
            msg.len() as libc::size_t,
        );
        Ok(())
    }
}

#[cfg(windows)]
#[doc(hidden)]
#[inline]
pub fn __libc_println(handle: i32, msg: &str) -> core::fmt::Result {
    unsafe {
        libc::write(
            handle,
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
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDOUT);
            stm.write_fmt(format_args!($($arg)*));
            stm.write_nl();
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
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDOUT);
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
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
            stm.write_fmt(format_args!($($arg)*));
            stm.write_nl();
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
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
            stm.write_fmt(format_args!($($arg)*));
        }
    };
}

/// Macro for printing a static string to the standard output.
///
/// Does not panic on failure to write - instead silently ignores errors.
#[macro_export]
macro_rules! libc_write {
    ($arg:expr) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDOUT);
            stm.write_str($arg);
        }
    }
}

/// Macro for printing a static string to the standard error.
///
/// Does not panic on failure to write - instead silently ignores errors.
#[macro_export]
macro_rules! libc_ewrite {
    ($arg:expr) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
            stm.write_str($arg);
        }
    }
}

/// Macro for printing a static string to the standard output, with a newline.
///
/// Does not panic on failure to write - instead silently ignores errors.
#[macro_export]
macro_rules! libc_writeln {
    ($arg:expr) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDOUT);
            stm.write_str($arg);
            stm.write_nl();
        }
    }
}

/// Macro for printing a static string to the standard error, with a newline.
///
/// Does not panic on failure to write - instead silently ignores errors.
#[macro_export]
macro_rules! libc_ewriteln {
    ($arg:expr) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
            stm.write_str($arg);
            stm.write_nl();
        }
    }
}

/// This package contains the `libc_print` macros, but using the stdlib names
/// such as `println!`, `print!`, etc.
pub mod std_name {
    /// Macro for printing to the standard output using the stdlib name.
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

    /// Macro for printing to the standard error, with a newline, using the stdlib name.
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

    /// Macro for printing to the standard error using the stdlib name.
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

    /// Macro for printing to the standard error, with a newline, using the stdlib name.
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

    #[cfg(test)]
    mod tests_std_name {
        #[test]
        fn test_stdout() {
            println!("stdout fd = {}", crate::__LIBC_STDOUT);
        }

        #[test]
        fn test_stderr() {
            eprintln!("stderr fd = {}", crate::__LIBC_STDERR);
        }
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

    #[test]
    fn test_stdout_write() {
        super::libc_writeln!("stdout!");
    }

    #[test]
    fn test_stderr_write() {
        super::libc_ewriteln!("stderr!");
    }
}
