# A Pet-feeding Game

## Introduction

This is a CLI project about playing a game of pet-feeding. You will have a pet which you can name. The pet will do the instruction you give it. You can feed it, bathe it, and play with it. You can also check its status.

## Usage

To run this application, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Then you can run the game by:

```bash
$ cargo run
```

Then you will be asked to choose the species of your pet. You can type whatever you want. After that, you will be asked to name your pet. You can also type whatever you want.

```bash
=====================================
Welcome to the pet-feeding game!
=====================================
Please choose a species for your pet:
You can type "dog", "cat", "bird", or "fish". Or other species you like.
dog
Please enter a name for your pet:
john
You have a pet named john and it is a dog.
=====================================
```

And you can interact with your pet.

```bash
        Please choose an action: 
        1. feed, 
        2. bath, 
        3. play, 
        4. check status, 
        5. instruct your pet to do something.
        
To quit the game, type "quit".
vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
1
You fed your dog john, and its food level is now 60.
```

Note that after you instruct your pet to do something, it will become hungrier, dirtier, and more bored.  
If the dog is either too hungry or too dirty or too bored, it will ask you to feed it, bathe it, or play with it.

```bash
        Please choose an action: 
        1. feed, 
        2. bath, 
        3. play, 
        4. check status, 
        5. instruct your pet to do something.
        
To quit the game, type "quit".
vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
5
Please enter the instruction.
walk
Your dog john is too dirty to walk.
```

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
