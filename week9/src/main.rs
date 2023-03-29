// a command-line tool to do math

use math::*;
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Wanqian",
    about = "Math command-line tool"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Wanqian")]
    Math {
        #[clap(short, long)]
        first: f64,
        #[clap(short, long)]
        second: f64,
        #[clap(short, long)]
        operator: String,
    },
}

fn main() {
    let args = Cli::parse();
    
    match args.command {
        Some(Commands::Math { first, second, operator }) => {
            let result = match operator.as_str() {
                "+" => add(first, second),
                "-" => subtract(first, second),
                "*" => multiply(first, second),
                "/" => divide(first, second),
                _ => panic!("Invalid operator"),
            };
            println!("{} {} {} = {}", first, operator, second, result);
        },
        None => println!("No command"),
    }
}