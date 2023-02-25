# Week6-mini-project: Minesweeper

## Description

This is a simple CLI minesweeper game. The goal is to find all the mines on the board without detonating any of them. You win if you find all cell without mines. You lose if you detonate a mine.

## Usage

To run the game, you need to have Rust installed. You can install Rust by following the instructions on [rust-lang.org](https://www.rust-lang.org/tools/install).

Then you can run the game by:

```bash
$ cargo run
```

To play the game, you first need to initialize the board. Simply follow the instructions on the screen.

```bash
Welcome to Minesweeper!
First, you need to initialize the board.
Then, you can click a cell to reveal it.
If you hit a mine, you lose.
If you reveal all the cells that are not mines, you win.
To quit the game, type "quit".
=====================================
To init: <rows> <cols> <mines>
3 3 1

  0 1 2 
0 ? ? ? 
1 ? ? ? 
2 ? ? ? 
```

Then you can click a cell to reveal it. 

```bash
=====================================
To click: <row> <col>
0 0
  0 1 2 
0 1 ? ? 
1 ? ? ? 
2 ? ? ? 
Continue!
```

If you win or lose, the game will end.

```bash
=====================================
To click: <row> <col>
1 2
  0 1 2 
0 1 1 1 
1 1 ? 1 
2 1 1 1 
You win!
1 1 1 
1 X 1 
1 1 1 
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
