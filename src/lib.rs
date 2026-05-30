//! Implements `println!`, `eprintln!` and `dbg!` on top of the `libc `crate without requiring
//! the use of an allocator.
//!
//! Allows you to use these macros in a #!\[no_std\] context, or in a situation where the
//! traditional Rust streams might not be available (ie: at process shutdown time).
//!
//! [`libc_writeln`] and [`libc_ewriteln`] are provided for cases where you may not wish
//! to pull in the overhead of the formatter code and simply wish to print C-style strings.
//!
//! ## Usage
//!
//! Exactly as you'd use `println!`, `eprintln!` and `dbg!`.
//!
//! ```rust
//! # use libc_print::*;
//! // Use the default `libc_`-prefixed macros:
//! # fn test1()
//! # {
//! libc_println!("Hello {}!", "stdout");
//! libc_eprintln!("Hello {}!", "stderr");
//! let a = 2;
//! let b = libc_dbg!(a * 2) + 1;
//! assert_eq!(b, 5);
//! # }
//! ```
//!
//! Or you can import aliases to `std` names:
//!
//! ```rust
//! use libc_print::std_name::{println, eprintln, dbg};
//!
//! # fn test2()
//! # {
//! println!("Hello {}!", "stdout");
//! eprintln!("Hello {}!", "stderr");
//! let a = 2;
//! let b = dbg!(a * 2) + 1;
//! assert_eq!(b, 5);
//! # }
//! ```

#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]

use core::convert::TryFrom;

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

#[doc(hidden)]
#[inline]
pub fn __libc_println(handle: i32, msg: &str) -> core::fmt::Result {
    let msg = msg.as_bytes();

    let mut written = 0;
    while written < msg.len() {
        match unsafe { libc_write(handle, &msg[written..]) } {
            // Ignore errors
            None | Some(0) => break,
            Some(res) => written += res,
        }
    }

    Ok(())
}

#[cfg(all(
    not(all(windows, miri)),
    not(any(all(target_family = "wasm", target_os = "unknown"), target_os = "none"))
))]
mod write {
    pub(crate) use libc::write;
}

#[cfg(any(all(target_family = "wasm", target_os = "unknown"), target_os = "none"))]
#[cfg_attr(all(target_family = "wasm", target_os = "unknown"), link(wasm_import_module = "env"))]
mod write {
    // The user is required to provide this
    unsafe extern "C" {
        pub(crate) fn write(fd: i32, buf: *const u8, nbyte: usize) -> isize;
    }
}

// Note: we may offer the lower-level WriteFile in the future.
#[cfg(all(windows, miri))]
mod write {
    use core::ffi::c_void;
    use core::sync::atomic::{AtomicPtr, Ordering};

    #[cfg(not(miri))]
    type BOOL = i32;
    type DWORD = u32;
    type ULONG = u32;
    type HANDLE = *mut c_void;
    type NTSTATUS = i32;

    const INVALID_HANDLE_VALUE: isize = -1;
    const STD_OUTPUT_HANDLE: DWORD = (-11i32) as DWORD;
    const STD_ERROR_HANDLE: DWORD = (-12i32) as DWORD;

    // https://learn.microsoft.com/windows-hardware/drivers/ddi/wdm/ns-wdm-_io_status_block
    // The first field is a pointer-sized union; model it as `usize`.
    #[cfg(miri)]
    #[cfg(target_pointer_width = "64")]
    #[repr(C)]
    struct IO_STATUS_BLOCK {
        status_or_ptr: usize,
        information: usize,
    }

    #[cfg(miri)]
    #[cfg(target_pointer_width = "32")]
    #[repr(C)]
    struct IO_STATUS_BLOCK {
        status_or_ptr: u32,
        information: u32,
    }

    unsafe extern "system" {
        fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;

        #[cfg(not(miri))]
        fn WriteFile(
            hFile: HANDLE,
            lpBuffer: *const c_void,
            nNumberOfBytesToWrite: DWORD,
            lpNumberOfBytesWritten: *mut DWORD,
            lpOverlapped: *mut c_void,
        ) -> BOOL;

        #[cfg(miri)]
        fn NtWriteFile(
            FileHandle: HANDLE,
            Event: HANDLE,
            ApcRoutine: *mut c_void,
            ApcContext: *mut c_void,
            IoStatusBlock: *mut IO_STATUS_BLOCK,
            Buffer: *const c_void,
            Length: ULONG,
            ByteOffset: *mut c_void,
            Key: *mut c_void,
        ) -> NTSTATUS;
    }

    #[inline]
    fn cached_std_handle(fd: i32) -> Option<(DWORD, &'static AtomicPtr<c_void>)> {
        static STD_OUTPUT: AtomicPtr<c_void> = AtomicPtr::new(INVALID_HANDLE_VALUE as _);
        static STD_ERROR: AtomicPtr<c_void> = AtomicPtr::new(INVALID_HANDLE_VALUE as _);

        match fd {
            1 => Some((STD_OUTPUT_HANDLE, &STD_OUTPUT)),
            2 => Some((STD_ERROR_HANDLE, &STD_ERROR)),
            _ => None,
        }
    }

    #[inline]
    fn handle_from_fd(fd: i32) -> Option<HANDLE> {
        let (std_handle, which) = cached_std_handle(fd)?;

        // Races are OK - we will get the same value each time assuming nobody
        // is calling SetStdHandle at the same time.
        let mut handle = which.load(Ordering::Relaxed);
        if handle.is_null() || handle as isize == INVALID_HANDLE_VALUE {
            handle = unsafe { GetStdHandle(std_handle) } as *mut c_void;
            if handle.is_null() || handle as isize == INVALID_HANDLE_VALUE {
                return None;
            }
            which.store(handle, Ordering::Relaxed);
        }

        Some(handle as HANDLE)
    }

    #[cfg(miri)]
    pub(crate) unsafe fn write(fd: i32, buf: *const u8, nbyte: usize) -> isize {
        let h = match handle_from_fd(fd) {
            Some(h) => h,
            None => return -1,
        };

        let len: ULONG = match ULONG::try_from(nbyte) {
            Ok(v) => v,
            Err(_) => ULONG::MAX,
        };

        let mut iosb = IO_STATUS_BLOCK {
            status_or_ptr: 0,
            information: 0,
        };

        let status = unsafe {
            NtWriteFile(
                h,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                core::ptr::null_mut(),
                &mut iosb,
                buf as *const c_void,
                len,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            )
        };

        // NT_SUCCESS: status >= 0
        if status < 0 {
            -1
        } else {
            iosb.information as isize
        }
    }

    #[cfg(not(miri))]
    pub(crate) unsafe fn write(fd: i32, buf: *const u8, nbyte: usize) -> isize {
        let h = match handle_from_fd(fd) {
            Some(h) => h,
            None => return -1,
        };

        let to_write: DWORD = match DWORD::try_from(nbyte) {
            Ok(v) => v,
            Err(_) => DWORD::MAX,
        };

        let mut written: DWORD = 0;
        let ok = unsafe {
            WriteFile(
                h,
                buf as *const c_void,
                to_write,
                &mut written,
                core::ptr::null_mut(),
            )
        };

        if ok == 0 {
            -1
        } else {
            written as isize
        }
    }
}

unsafe fn libc_write(handle: i32, bytes: &[u8]) -> Option<usize> {
    usize::try_from(unsafe { write::write(handle as _, bytes.as_ptr() as _, bytes.len() as _) })
        .ok()
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
    () => { $crate::libc_println!("") };
    ($($arg:tt)*) => {
        {
            #[allow(unused_must_use)]
            {
                let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDOUT);
                stm.write_fmt(format_args!($($arg)*));
                stm.write_nl();
            }
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
        {
            #[allow(unused_must_use)]
            {
                let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDOUT);
                stm.write_fmt(format_args!($($arg)*));
            }
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
    () => { $crate::libc_eprintln!("") };
    ($($arg:tt)*) => {
        {
            #[allow(unused_must_use)]
            {
                let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
                stm.write_fmt(format_args!($($arg)*));
                stm.write_nl();
            }
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
        {
            #[allow(unused_must_use)]
            {
                let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
                stm.write_fmt(format_args!($($arg)*));
            }
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
    };
}

/// Macro for printing a static string to the standard error.
///
/// Does not panic on failure to write - instead silently ignores errors.
#[macro_export]
macro_rules! libc_ewrite {
    ($arg:expr) => {{
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
            stm.write_str($arg);
        }
    }};
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
    };
}

/// Macro for printing a static string to the standard error, with a newline.
///
/// Does not panic on failure to write - instead silently ignores errors.
#[macro_export]
macro_rules! libc_ewriteln {
    ($arg:expr) => {{
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::__LibCWriter::new($crate::__LIBC_STDERR);
            stm.write_str($arg);
            stm.write_nl();
        }
    }};
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging.
///
/// An example:
///
/// ```rust
/// let a = 2;
/// let b = dbg!(a * 2) + 1;
/// //      ^-- prints: [src/main.rs:2] a * 2 = 4
/// assert_eq!(b, 5);
/// ```
///
/// See [dbg!](https://doc.rust-lang.org/std/macro.dbg.html) for full documentation.
///
/// You may wish to `use libc_print::std_name::*` to use a replacement
/// `dbg!` macro instead of this longer name.
#[macro_export]
macro_rules! libc_dbg {
    () => {
        $crate::libc_eprintln!("[{}:{}]", ::core::file!(), ::core::line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::libc_eprintln!("[{}:{}] {} = {:#?}", ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::libc_dbg!($val)),+,)
    };
}

/// This package contains the `libc_print` macros, but using the stdlib names
/// such as `println!`, `print!`, etc.
pub mod std_name {
    pub use super::libc_dbg as dbg;
    pub use super::libc_eprint as eprint;
    pub use super::libc_eprintln as eprintln;
    pub use super::libc_print as print;
    pub use super::libc_println as println;

    #[cfg(test)]
    mod tests_std_name {
        use super::{eprintln, println};

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

    #[test]
    fn test_dbg() {
        let a = 2;
        let b = libc_dbg!(a * 2) + 1;
        assert_eq!(b, 5);
    }

    #[test]
    fn test_in_closure_expression() {
        use super::std_name::*;
        // https://github.com/mmastrac/rust-libc-print/issues/86
        let _ = Result::<(), ()>::Ok(()).unwrap_or_else(|err| eprintln!("error: {:?}", err));
    }
}
