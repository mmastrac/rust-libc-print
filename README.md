# `#![no_std]` libc print/println/eprint/eprintln

[![Build Status](https://api.travis-ci.org/mmastrac/rust-libc-print.svg?branch=master)](https://travis-ci.org/mmastrac/rust-libc-print)
[![docs.rs](https://docs.rs/libc-print/badge.svg)](https://docs.rs/libc-print)
[![crates.io](https://img.shields.io/crates/v/libc-print.svg)](https://crates.io/crates/libc-print)

Implements `println!` and `eprintln!` on the `libc` crate without 
requiring the use of an allocator.

Allows you to use these macros in a `#![no_std]` context, or in a 
situation where the traditional Rust streams might not be available 
(ie: at process shutdown time).

## Usage

Exactly as you'd use `println!` or `eprintln!`.

