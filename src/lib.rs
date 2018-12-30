#![no_std]
#![allow(dead_code)]

pub const __LIBC_NEWLINE: &str = "\n";
pub const __LIBC_STDOUT: u32 = 1;
pub const __LIBC_STDERR: u32 = 2;

struct LibCWriter(u32);

impl core::fmt::Write for LibCWriter {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __libc_println(self.0, s)
    }
}

impl LibCWriter {
    #[inline]
    fn write_fmt(&mut self, args: core::fmt::Arguments) -> core::fmt::Result {
        core::fmt::Write::write_fmt(self, args)
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        core::fmt::Write::write_str(self, s)
    }
}

#[cfg(not(windows))]
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
pub fn __libc_println(handle: u32, msg: &str) -> core::fmt::Result {
    unsafe {
        libc::write(handle as i32, msg.as_ptr() as *const core::ffi::c_void, msg.len() as u32);
        Ok(())
    }
}

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

#[macro_use]
pub mod std_name {
    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => {
            $crate::libc_print!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {
            $crate::libc_println!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! eprint {
        ($($arg:tt)*) => {
            $crate::libc_eprint!($($arg)*);
        };
    }

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
