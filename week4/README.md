# Week 4 Mini Project for IDS 721

## Introduction

This is a mini project of command line application for Week 4 of IDS 721. The purpose of this mini project is to create a command line application using Rust, which can be used to track budget.

## Usage

To run this application, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Also, you have to install Cargo, which is Rust's package manager. You can find instructions on how to install Cargo [here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Once you have them installed, you can run the application by running the following command:

```cargo run```

Then follow the instructions on the screen, which looks like this:

```bash
    1. Add a transaction -> add + type(1 for income, -1 for expense) + amount + description
    2. Show all transactions -> list
    3. Show income details -> income
    4. Show expense details -> expenses
    5. Show balance -> balance
    6. Delete a transaction -> delete + index
    7. Quit -> quit
```

Please feel free to follow the instructions and try all the features.

## Examples

```bash
    add 1 200 test
```

This command will **add a transaction** with type 1 (income), amount 200, and description "test".

```bash
    list
```

This command will **list all the transactions**. And the return would be like:
    
```bash
    0. Income: 200, Description: test
```

After adding more transactions, the return would be like:

```bash
    0. Income: 200, Description: test
    1. Expense: 100, Description: test2
    2. Income: 300, Description: test3
```

If you want to get the **income details**, you can run:

```bash
    income
```

The return would be like:

```bash
    0. Income: 200, Description: test
    1. Income: 300, Description: test3
```

And if you want to **get the balance**, you can run:

```bash
    balance
```

The return would be like:

```bash
    Balance: 400
```

And finally, if you want to **delete a transaction**, you can run:

```bash
    delete 1
```

The return would be like:

```bash
    0. Income: 200, Description: test
    1. Income: 300, Description: test3
```
