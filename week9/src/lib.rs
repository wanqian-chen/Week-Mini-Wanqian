// provide basic functions to play paper-scissors-rock game

use rand::Rng;

// get a random number
pub fn get_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen::<u32>() % 3; // generate a random number between 0 and 2
    random_number
}

// get a random choice
pub fn get_random_choice() -> String {
    let choices = vec!["paper", "scissors", "rock"];
    let choice = get_random_number();
    choices[choice as usize].to_string()
}

// check who wins
pub fn check_who_wins(choice1: &str, choice2: &str) -> String {
    if choice1 == choice2 {
        "Draw!".to_string()
    } else if choice1 == "paper" && choice2 == "rock" {
        "Congrats! You win!".to_string()
    } else if choice1 == "paper" && choice2 == "scissors" {
        "Sorry! You lose!".to_string()
    } else if choice1 == "scissors" && choice2 == "paper" {
        "Congrats! You win!".to_string()
    } else if choice1 == "scissors" && choice2 == "rock" {
        "Sorry! You lose!".to_string()
    } else if choice1 == "rock" && choice2 == "scissors" {
        "Congrats! You win!".to_string()
    } else if choice1 == "rock" && choice2 == "paper" {
        "Sorry! You lose!".to_string()
    } else {
        "Something went wrong!".to_string()
    }
}

// play the paper-scissors-rock game
pub fn paper_scissors_rock(choice: String) -> String {
    let random_choice = get_random_choice();
    let result = check_who_wins(&choice, &random_choice);
    let output = format!("You chose {}. The computer chose {}. {}", choice, random_choice, result);
    output
}
