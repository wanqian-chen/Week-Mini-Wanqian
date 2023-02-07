//A command-line tool to play hello_somebody

use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Wanqian", about = "Throw a coin")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Wanqian")]
    Coin {
        #[clap(short, long)]
        probability: f64,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Coin { probability }) => {
            let result = coin::coin(probability);
            println!("{}", result);
        }
        None => println!("Something went wrong!"),
    }
}
