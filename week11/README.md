# Shopping

## Introduction

This is a CLI project about shopping, which can calculate the total price of the items you bought.

## Usage

To run this application, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Then you can run the game by:

```bash
$ cargo run
```

You will get the menu:

```bash
Welcome to the order system!
====================
What would you like to do?
add <name> <price> <quantity>
list
quit
====================
```

Feel free to follow the instruction and try it out!

## Example

To add items:

```bash
add milk 3 5
add apple 1 3
```

To list items and get the total price:

```bash
====================
list
0. milk 3 5
1. apple 1 3
Total: 18
====================
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
