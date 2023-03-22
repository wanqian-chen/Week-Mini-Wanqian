// a command-line tool to play pet-feeding

use std::io;
use clap::Parser;

use pet_feeding::{init_pet, feed_pet, give_pet_a_bath, play_with_pet, check_pet_status};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Wanqian",
    about = "Play pet-feeding"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Wanqian")]
    PetFeeding {
        #[clap(short, long)]
        name: String,
        #[clap(short, long)]
        species: String,
    },
}

fn main() {
    let args = Cli::parse();
    // welcome message
    println!("=====================================");
    println!("Welcome to the pet-feeding game!");
    println!("=====================================");
    
    // initialize a pet
    println!("Please enter your pet's name and species.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let species = input.trim().to_string();
    let mut pet = init_pet(name, species);
    println!("You have a pet named {} and it is a {}.", pet.name, pet.species);
    println!("=====================================");
    
    loop {
        println!("Please choose an action: 1. feed, 2. bath, 3. play, 4. check status, 5. instruct your pet to do something.");
        println!("To quit the game, type \"quit\".");
        // get the choice
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_string();
        // play the game
        if choice == "quit" {
            println!("You quit the game.");
            break;
        }
        if choice == "1" {
            pet = feed_pet(pet);
        } else if choice == "2" {
            pet = give_pet_a_bath(pet);
        } else if choice == "3" {
            pet = play_with_pet(pet);
        } else if choice == "4" {
            check_pet_status(pet);
        } else if choice == "5" {
            println!("Please enter what you want your pet to do.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let action = input.trim().to_string();
            pet = pet.do_something(action);
        } else {
            println!("Invalid choice.");
        }

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

