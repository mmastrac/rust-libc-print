# no_std libc print/println/eprint/eprintln/dbg

![Build Status](https://github.com/mmastrac/rust-libc-print/actions/workflows/rust.yml/badge.svg)
[![docs.rs](https://docs.rs/libc-print/badge.svg)](https://docs.rs/libc-print)
[![crates.io](https://img.shields.io/crates/v/libc-print.svg)](https://crates.io/crates/libc-print)

Implements `println!`, `eprintln!` and `dbg!` on the `libc` crate without 
requiring the use of an allocator.

Allows you to use these macros in a `#![no_std]` context, or in a 
situation where the traditional Rust streams might not be available 
(ie: at process shutdown time).

By default this crate provides `libc_`-prefixed macros, but also allows consumers to
import macros with the same name as the stdlib printing macros via the `std_name`
module.

## Usage

Exactly as you'd use `println!`, `eprintln!` and `dbg!`.

```rust
#![no_std]

// Use the default `libc_`-prefixed macros:
libc_println!("Hello {}!", "stdout");
libc_eprintln!("Hello {}!", "stderr");
let a = 2;
let b = libc_dbg!(a * 2) + 1;
assert_eq!(b, 5);
```

Or you can import aliases to `std` names:

```rust
use libc_print::std_name::{println, eprintln, dbg};

println!("Hello {}!", "stdout");
eprintln!("Hello {}!", "stderr");
let a = 2;
let b = dbg!(a * 2) + 1;
assert_eq!(b, 5);
```
