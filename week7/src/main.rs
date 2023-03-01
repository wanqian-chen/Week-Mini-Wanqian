// a command-line tool to play paper-scissors-rock

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
    println!("Welcome to the paper-scissors-rock game!");
    // menu
    println!("Please choose your weapon: paper, scissors, or rock.");
    println!("To quit the game, type \"quit\".");
    println!("=====================================");
    match args.command {
        Some(Commands::PaperScissorsRock { choice }) => {
            let result = paper_scissors_rock::paper_scissors_rock(choice);
            println!("{}", result);
        }
        None => println!("Something went wrong!"),
    }
}
