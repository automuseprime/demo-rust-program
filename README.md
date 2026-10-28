# demo-rust-program

A small demo Rust program, created as a sample repo.

It greets you, counts to 5, and shows a tiny bit of ownership
and error handling, with a few unit tests.

## Run it

```sh
cargo run -- Paul
```

Expected output (abridged):

```
Hello, Paul! This is a demo Rust program.

Counting to 5:
  1
  2
  3
  4
  5

HELLO FROM PAUL!

Parsed 42 as: 42 (doubled: 84)
Could not parse 'not-a-number': invalid digit found in string
```

## Test it

```sh
cargo test
```

## Requirements

Rust toolchain (`cargo` and `rustc`), e.g. via rustup: https://rustup.rs
