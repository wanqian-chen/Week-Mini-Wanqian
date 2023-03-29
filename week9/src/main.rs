// a command-line tool to play paper-scissors-rock

use std::io;
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Wanqian",
    about = "Play paper-scissors-rock"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Wanqian")]
    PaperScissorsRock {
        #[clap(short, long)]
        choice: String,
    },
}

fn main() {
    let args = Cli::parse();
    // welcome message
    println!("=====================================");
    println!("Welcome to the paper-scissors-rock game!");
    println!("To quit the game anytime, type \"quit\".");
    println!("=====================================");
    
    loop {
        println!("Please choose your weapon: paper, scissors, or rock.");
        // get the choice
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_string();
        // play the game
        if choice == "quit" {
            println!("You quit the game.");
            break;
        }
        let result = paper_scissors_rock::paper_scissors_rock(choice);
        println!("{}", result);

        // ask if want to continue
        println!("Do you want to continue? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let choice_continue = input.trim().to_string();
        if choice_continue == "n" {
            println!("You quit the game.");
            break;
        }
        println!("=====================================");
    }
}