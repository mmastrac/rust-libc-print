#![no_std]
#![allow(dead_code)]

pub const __NOSTD_NEWLINE: &str = "\n";
pub const __NOSTD_STDOUT: u32 = 1;
pub const __NOSTD_STDERR: u32 = 2;

struct NoStdWriter(u32);

impl core::fmt::Write for NoStdWriter {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        __nostd_println(self.0, s)
    }
}

impl NoStdWriter {
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
pub fn __nostd_println(handle: u32, msg: &str) -> core::fmt::Result {
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
pub fn __nostd_println(handle: u32, msg: &str) -> core::fmt::Result {
    unsafe {
        libc::write(handle as i32, msg.as_ptr() as *const core::ffi::c_void, msg.len() as u32);
        Ok(())
    }
}

#[macro_export]
macro_rules! nostd_println {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::NoStdWriter($crate::__NOSTD_STDOUT);
            stm.write_fmt(format_args!($($arg)*));
            stm.write_str($crate::__NOSTD_NEWLINE);
        }
    };
}

#[macro_export]
macro_rules! nostd_print {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::NoStdWriter($crate::__NOSTD_STDOUT);
            stm.write_fmt(format_args!($($arg)*));
        }
    };
}

#[macro_export]
macro_rules! nostd_eprintln {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::NoStdWriter($crate::__NOSTD_STDERR);
            stm.write_fmt(format_args!($($arg)*));
            stm.write_str($crate::__NOSTD_NEWLINE);
        }
    };
}

#[macro_export]
macro_rules! nostd_eprint {
    ($($arg:tt)*) => {
        #[allow(unused_must_use)]
        {
            let mut stm = $crate::NoStdWriter($crate::__NOSTD_STDERR);
            stm.write_fmt(format_args!($($arg)*));
        }
    };
}

#[macro_use]
pub mod std_name {
    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => {
            $crate::nostd_print!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => {
            $crate::nostd_println!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! eprint {
        ($($arg:tt)*) => {
            $crate::nostd_eprint!($($arg)*);
        };
    }

    #[macro_export]
    macro_rules! eprintln {
        ($($arg:tt)*) => {
            $crate::nostd_eprintln!($($arg)*);
        };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stdout() {
        super::nostd_println!("stdout fd = {}", super::__NOSTD_STDOUT);
    }

    #[test]
    fn test_stderr() {
        super::nostd_eprintln!("stderr fd = {}", super::__NOSTD_STDERR);
    }
}

#[cfg(test)]
mod tests_std_name {
    #[test]
    fn test_stdout() {
        println!("stdout fd = {}", super::__NOSTD_STDOUT);
    }

    #[test]
    fn test_stderr() {
        eprintln!("stderr fd = {}", super::__NOSTD_STDERR);
    }
}
