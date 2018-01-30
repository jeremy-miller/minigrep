[![MIT Licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/jeremy-miller/minigrep/blob/master/LICENSE)
[![Rust Version](https://img.shields.io/badge/Rust-1.23.0-blue.svg)]()

# minigrep
Command line tool for searching a file for specific text.
This tool is based on Chapter 12 of the
[Rust book (second edition)](https://doc.rust-lang.org/book/second-edition/ch12-00-an-io-project.html).

# Prerequisites
This tool requires Rust version 1.23.0 or higher to be installed.

# Bulid
To build `minigrep`, execute the following command:
```cargo build```

# Test
To run the `minigrep` tests, execute the following command:
```cargo test```

# Run
This tool can perform both case-sensitive (default) and case-insensitive searches of a file for text.
To perform a case-insensitive search, you can set the `CASE_INSENSITIVE` environment variable to any value.

To run a `minigrep` search, execute the following command (substituting values for `<search string>` and `<file to search>`):
```cargo run <search string> <file to search>```

## Case-Sensitive Search Example
Below is an example of a case-sensitive search using `minigrep`.
```
$ cargo run to poem.txt
  Finished dev [unoptimized + debuginfo] target(s) in 0.85 secs
   Running `target/debug/minigrep to poem.txt`
Are you nobody, too?
How dreary to be somebody!
```

## Case-Insensitive Search Example
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
