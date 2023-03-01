# A Paper-Scissors-Rock Game

## Introduction

This is a CLI project about playing a game of paper-scissors-rock. The game will be played against the computer.

## Usage

To run this application, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Then you can run the game by:

```bash
$ cargo run
```

Then you will be asked to choose one of the three options: paper, scissors, or rock. The computer will randomly choose one of the three options and the result will be shown.

```bash
=====================================
Welcome to the paper-scissors-rock game!
To quit the game anytime, type "quit".
=====================================
Please choose your weapon: paper, scissors, or rock.
```

And it will return the result of the game.

```bash
You chose scissors. The computer chose rock. Sorry! You lose!
```

It will then ask if the user wants to play again.

```bash
Do you want to continue? (y/n)
```

y: continue the game
n: quit the game

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
