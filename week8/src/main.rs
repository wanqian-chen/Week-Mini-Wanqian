// a command-line tool to play pet-feeding

use clap::Parser;
use std::io;

use pet_feeding::{check_pet_status, feed_pet, give_pet_a_bath, init_pet, play_with_pet, instruct_pet};

#[derive(Parser)]
#[clap(version = "1.0", author = "Wanqian", about = "Play pet-feeding")]
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
    println!("Please choose a species for your pet:");
    println!("You can type \"dog\", \"cat\", \"bird\", or \"fish\". Or other species you like.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let species = input.trim().to_string();
    println!("Please enter a name for your pet:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let name = input.trim().to_string();
    let mut pet = init_pet(name, species);
    println!(
        "You have a pet named {} and it is a {}.",
        pet.name, pet.species
    );
    println!("=====================================");

    loop {
        println!("
        Please choose an action: 
        1. feed, 
        2. bath, 
        3. play, 
        4. check status, 
        5. instruct your pet to do something.
        ");
        println!("To quit the game, type \"quit\".");
        println!("=====================================");
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
            feed_pet(&mut pet);
        } else if choice == "2" {
            give_pet_a_bath(&mut pet);
        } else if choice == "3" {
            play_with_pet(&mut pet);
        } else if choice == "4" {
            check_pet_status(&mut pet);
        } else if choice == "5" {
            println!("Please enter the instruction.");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let instruction = input.trim().to_string();
            instruct_pet(&mut pet, instruction);
        }
        else {
            println!("Invalid input.");
        }

        println!("=====================================");
    }
}
