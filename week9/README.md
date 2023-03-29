# Basic Math

## Introduction

This is a CLI project about doing basic math

## Usage

To run this application, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Then you can run the game by:

```bash
$ cargo run -- math --first <first-number> --second <second-number> --operator <operation>
```

A operation can be one of the following:
"+", "-", "*", "/"

## Example

```bash
$ cargo run -- math --first 5 --second 2 --operator +
```

Result:

```bash
5 + 2 = 7
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
