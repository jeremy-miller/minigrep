[![Build Status](https://travis-ci.org/jeremy-miller/minigrep.svg?branch=master)](https://travis-ci.org/jeremy-miller/minigrep)
[![codecov](https://codecov.io/gh/jeremy-miller/minigrep/branch/master/graph/badge.svg)](https://codecov.io/gh/jeremy-miller/minigrep)
[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jeremy-miller/minigrep/blob/master/LICENSE)
[![Rust Version](https://img.shields.io/badge/Rust-1.23.0-blue.svg)]()

# Minigrep
Command line tool for searching a file for specific text.
This tool is based on chapter 12 of the
[Rust book (second edition)](https://doc.rust-lang.org/book/second-edition/ch12-00-an-io-project.html).

## Table of Contents
- [Motivation](#motivation)
- [Prerequisites](#prerequisites)
- [Documentation](#documentation)
- [Build](#build)
- [Code Formatting](#code-formatting)
- [Static Code Analysis](#static-code-analysis)
- [Test](#test)
- [Run](#run)
    - [Case-Sensitive Search Example](#case-sensitive-search-example)
    - [Case-Insensitive Search Example](#case-insensitive-search-example)
- [License](#license)

## Motivation
I created this project while reading the [Rust book (second edition)](https://doc.rust-lang.org/book/second-edition/).

## Prerequisites
This tool requires at least Rust version 1.23.0 to be installed.

## Documentation
To build and view the `minigrep` documentation in your browser, execute the following command:
```cargo doc --open```

## Build
To build `minigrep`, execute the following command:
```cargo build --all```

## Code Formatting
To run `rustfmt` on `minigrep`, execute the following steps:

1. Install `rustfmt` (requires rust `nightly`): ```rustup component add rustfmt-preview --toolchain=nightly```
2. Run `rustfmt` on the `minigrep` codebase: ```cargo +nightly fmt```

## Static Code Analysis
To run `clippy` on `minigrep`, execute the following steps:

1. Install `clippy` (requires rust `nightly`): ```cargo +nightly install clippy```
2. Run `clippy` on the `minigrep` codebase: ```cargo +nightly clippy```

## Test
To run the `minigrep` tests, execute the following command:
```cargo test --all```

## Run
This tool can perform both case-sensitive (default) and case-insensitive searches of a file for text.
To perform a case-insensitive search, you can set the `CASE_INSENSITIVE` environment variable to any value.

To run a `minigrep` search, execute the following command (substituting values for `<search string>` and `<file to search>`):
```cargo run <search string> <file to search>```

### Case-Sensitive Search Example
Below is an example of a case-sensitive search using `minigrep`.
```
$ cargo run to poem.txt
  Finished dev [unoptimized + debuginfo] target(s) in 0.85 secs
   Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

### Case-Insensitive Search Example
Below is an example of a case-insensitive search using `minigrep`.
Note the `CASE_INSENSITIVE` environment variable passed as part of the command.
```
$ CASE_INSENSITIVE=1 cargo run to poem.txt 
  Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
   Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

## License
[MIT](https://github.com/jeremy-miller/minigrep/blob/master/LICENSE)
