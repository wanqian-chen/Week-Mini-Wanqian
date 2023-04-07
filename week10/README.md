# To-do calendar

## Introduction

This is a CLI project about todolist calendar.

## Usage

To run this application, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Then you can run the game by:

```bash
$ cargo run
```

You will get the menu:

```bash
What would you like to do?
1. Add a task -> add + task + date(yyyy-mm-dd)
2. Mark a task as done -> done + index
3. List all tasks -> list
4. List today's tasks -> today
5. Quit -> quit
==============================
```

Feel free to follow the instruction and try it out!

## Example

To add tasks:

```bash
What would you like to do?
1. Add a task -> add + task + date(yyyy-mm-dd)
2. Mark a task as done -> done + index
3. List all tasks -> list
4. List today's tasks -> today
5. Quit -> quit
==============================
add go-to-work 2023-04-01
==============================
What would you like to do next?
1. Add a task -> add + task + date(yyyy-mm-dd)
2. Mark a task as done -> done + index
3. List all tasks -> list
4. List today's tasks -> today
5. Quit -> quit
==============================
add pay-bill 2023-04-02
```

To list all tasks:

```bash
==============================
list
0. go-to-work 2023-04-01. 
1. pay-bill 2023-04-02. 
==============================
```

To list today's tasks:

```bash
==============================
today
1. pay-bill 2023-04-02. 
==============================
```

To mark a task as done:

```bash
==============================
done 0
==============================
```

```bash
==============================
list
0. pay-bill 2023-04-02. 
==============================
```

To quit:

```bash
quit
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
