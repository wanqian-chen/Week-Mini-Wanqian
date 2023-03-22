// provide basic functions to play pet game

// struct to store pet information
pub struct Pet {
    pub name: String,
    pub species: String,
    pub food: u32,
    pub bath: u32,
    pub play: u32,
}

// initialize a pet
pub fn init_pet(name: String, species: String) -> Pet {
    let pet = Pet {
        name,
        species,
        food: 50,
        bath: 50,
        play: 50,
    };
    pet
}

// feed a pet
pub fn feed_pet(pet: &mut Pet) {
    // if food is less than 90, add 10
    if pet.food <= 90 {
        pet.food += 10;
    }
    // if food is more than 90, set food to 100
    if pet.food > 90 {
        pet.food = 100;
    }
    println!(
        "You fed your {} {}, and its food level is now {}.",
        pet.species, pet.name, pet.food
    );
}

// give a pet a bath
pub fn give_pet_a_bath(pet: &mut Pet) {
    // if bath is less than 90, add 10
    if pet.bath <= 90 {
        pet.bath += 10;
    }
    // if bath is more than 90, set bath to 100
    if pet.bath > 90 {
        pet.bath = 100;
    }
    println!(
        "You gave your {} {} a bath, and its bath level is now {}.",
        pet.species, pet.name, pet.bath
    );
}

// play with a pet
pub fn play_with_pet(pet: &mut Pet) {
    // if play is less than 90, add 10
    if pet.play <= 90 {
        pet.play += 10;
    }
    // if play is more than 90, set play to 100
    if pet.play > 90 {
        pet.play = 100;
    }
    println!(
        "You played with your {} {}, and its play level is now {}.",
        pet.species, pet.name, pet.play
    );
}

// check the status of a pet
pub fn check_pet_status(pet: &mut Pet) {
    // if food is less than 50, notify the user
    if pet.food < 50 {
        println!(
            "Your {} {} is hungry. You should feed it.",
            pet.species, pet.name
        );
    }
    // if bath is less than 50, notify the user
    if pet.bath < 50 {
        println!(
            "Your {} {} is dirty. You should give it a bath.",
            pet.species, pet.name
        );
    }
    // if play is less than 50, notify the user
    if pet.play < 50 {
        println!(
            "Your {} {} is bored. You should play with it.",
            pet.species, pet.name
        );
    }
    // otherwise, the pet is happy
    if pet.food >= 50 && pet.bath >= 50 && pet.play >= 50 {
        println!("Your {} {} is happy.", pet.species, pet.name);
        println!("Your pet has food level {}, bath level {}, and play level {}.", pet.food, pet.bath, pet.play);
    }
}

// instruct the pet
pub fn instruct_pet(pet: &mut Pet, instruction: String) {
    // if any num is less than 5, notify the user
    if pet.food < 5 {
        println!(
            "Your {} {} is too hungry to {}.",
            pet.species, pet.name, instruction
        );
    } else if pet.bath < 5 {
        println!(
            "Your {} {} is too dirty to {}.",
            pet.species, pet.name, instruction
        );
    } else if pet.play < 5 {
        println!(
            "Your {} {} is too bored to {}.",
            pet.species, pet.name, instruction
        );
    } else {
        // do the instruction
        println!("Your {} {} {}.", pet.species, pet.name, instruction);
        // decrease food, bath, and play by 5
        pet.food -= 5;
        pet.bath -= 5;
        pet.play -= 5;
    }
}
