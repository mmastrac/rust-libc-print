# `#[nostd]` libc print/println/eprint/eprintln

[![Build Status](https://api.travis-ci.org/mmastrac/rust-libc-print.svg?branch=master)](https://travis-ci.org/mmastrac/rust-libc-print)
[![docs.rs](https://docs.rs/libc-print/badge.svg)](https://docs.rs/libc-print)
[![crates.io](https://img.shields.io/crates/v/libc-print.svg)](https://crates.io/crates/libc-print)

Implements `println!` and `eprintln!` on the `libc` crate without 
requiring the use of an allocator.

## Usage

Exactly as you'd use `println!` or `eprintln!`.

