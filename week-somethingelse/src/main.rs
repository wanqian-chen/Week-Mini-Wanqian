use caffeine_calculator::*;
use std::io;

fn main() {
    let mut input = String::new();
    println!("=====================================");
    println!("Welcome to caffeine calculator!");
    println!("Please choose from the following options:");
    println!("1. Drink a cup of coffee");
    println!("2. See your drinking history");
    println!("=====================================");
    let your_caffeine_history = init_history();
    let all_coffee = init_coffee();

    loop {
        println!("Please enter your choice:");
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim().to_string();
        if choice == "1" {
            println!("=====================================");
            println!("We provide serveral types of coffee:");
            show_coffee(all_coffee);
            println!("Do you find the coffee you like?(y/n)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let choice = input.trim().to_string();
            if choice == "y" {
                println!("Please enter the name of the coffee:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let coffee_name = input.trim().to_string();
                println!("Please enter the size of the coffee:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                // get coffee size u32
                let coffee_size = input.trim().parse::<u32>().unwrap();
                drink(coffee_name, coffee_size, &mut your_caffeine_history, all_coffee);
            } else {
                println!("Please enter the coffee name:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let coffee_name = input.trim().to_string();
                println!("Please enter the caffeine per ounce:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let caffeine_per_ounce = input.trim().parse::<f32>().unwrap();
                println!("Please enter the size of the coffee:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let coffee_size = input.trim().parse::<u32>().unwrap();
                let drink_coffee = drink_custom(coffee_name, coffee_size, caffeine_per_ounce, &mut your_caffeine_history);
                println!("=====================================");
                println!("Do you want to add your coffee to our menu?(y/n)");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let choice_add = input.trim().to_string();
                if choice_add == "y" {
                    all_coffee = add_coffee(drink_coffee.category, &mut all_coffee).to_vec();
                    println!("Your coffee has been added to our menu.");
                }
                else {
                    println!("Sure. We will not add your coffee to our menu.");
                }
            }
        } else if choice == "2" {
            show(&your_caffeine_history);
        } else {
            println!("Invalid choice. Please try again.");
        }
    }
}

