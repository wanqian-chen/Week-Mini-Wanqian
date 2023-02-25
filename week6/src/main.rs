// A CLI minesweeper game

use clap::Parser;

// import from lib.rs
use minesweeper::{user_click, user_init, Board};

#[derive(Parser)]
#[clap(version = "1.0", author = "Wanqian", about = "Minesweeper")]

struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Wanqian")]
    Init {
        #[clap(short, long)]
        rows: usize,
        #[clap(short, long)]
        cols: usize,
        #[clap(short, long)]
        mines: usize,
    },
    Click {
        #[clap(short, long)]
        row: usize,
        #[clap(short, long)]
        col: usize,
    },
}

fn main() {
    let args = Cli::parse();

    // Welcome message
    println!("Welcome to Minesweeper!");
    // print the menu
    println!("First, you need to initialize the board.");
    println!("Then, you can click a cell to reveal it.");
    println!("If you hit a mine, you lose.");
    println!("If you reveal all the cells that are not mines, you win.");
    println!("To quit the game, type \"quit\".");
    println!("=====================================");

    let mut init = false;

    println!("To init: <rows> <cols> <mines>");
    // get user input
    let mut board = Board {
        board: vec![],
        rows: 0,
        cols: 0,
        mines: 0,
    };

    // get user input
    let mut input_init = String::new();
    std::io::stdin().read_line(&mut input_init).unwrap();
    if input_init.trim() == "quit" {
        println!("You quit the game.");
        return;
    }
    let input_init: Vec<&str> = input_init.split_whitespace().collect();
    // get rows, cols, mines
    let rows = input_init[0].parse::<usize>().unwrap();
    let cols = input_init[1].parse::<usize>().unwrap();
    let mines = input_init[2].parse::<usize>().unwrap();

    // initialize the board
    let mut board = user_init(rows, cols, mines);

    // loop until the game is over
    loop {
        println!("=====================================");
        println!("To click: <row> <col>");
        // get user input
        let mut input_click = String::new();
        std::io::stdin().read_line(&mut input_click).unwrap();
        if input_click.trim() == "quit" {
            println!("You quit the game.");
            break;
        }
        let input_click: Vec<&str> = input_click.split_whitespace().collect();
        // get row and col
        let row = input_click[0].parse::<usize>().unwrap();
        let col = input_click[1].parse::<usize>().unwrap();
        // click the cell
        if user_click(&mut board, row, col) {
            break;
        }
    }
}
