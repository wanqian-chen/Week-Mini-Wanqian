#[derive(Clone)]
pub struct BasicCoffee {
    pub name: String,
    pub caffeine_per_ounce: f32,
}

pub struct Coffee {
    pub category: BasicCoffee,
    pub ounces: u32,
}

// store caffeine history
pub struct CaffeineHistory {
    pub history: Vec<Coffee>,
    pub total_caffeine: f32,
}

// drink coffee
pub fn drink(name: String, ounces: u32, history: &mut CaffeineHistory, all_coffee: Vec<BasicCoffee>) {
    // find the coffee
    let coffee = all_coffee.iter().find(|c| c.name == name).unwrap();
    // add to caffeine history
    let new_history = Coffee {
        category: (*coffee).clone(),
        ounces: ounces,
    };
    history.history.push(new_history);
    // update total caffeine
    history.total_caffeine += coffee.caffeine_per_ounce * ounces as f32;
    println!("You have drunk {} oz of {} coffee", ounces, name);
}

// show caffeine history
pub fn show(history: &CaffeineHistory) {
    // print total caffeine
    println!("Total caffeine: {} mg", history.total_caffeine);
    // print caffeine history
    for (index, coffee) in history.history.iter().enumerate() {
        println!("{}. {} oz of {} coffee", index, coffee.ounces, coffee.category.name);
    }
}

// show existing coffee and their caffeine content
pub fn show_coffee(all_coffee: Vec<BasicCoffee>) {
    for (index, coffee) in all_coffee.iter().enumerate() {
        println!("{}. {} coffee: {} mg/oz", index, coffee.name, coffee.caffeine_per_ounce);
    }
}

// a function to initialize caffeine history
pub fn init_history() -> CaffeineHistory {
    CaffeineHistory {
        history: Vec::new(),
        total_caffeine: 0.0,
    }
}

// a function to initialize all coffee
pub fn init_coffee() -> Vec<BasicCoffee> {
    let mut all_coffee = Vec::new();
    all_coffee.push(BasicCoffee {
        name: "latte".to_string(),
        caffeine_per_ounce: 6.92,
    });
    all_coffee.push(BasicCoffee {
        name: "americano".to_string(),
        caffeine_per_ounce: 12.83,
    });
    all_coffee.push(BasicCoffee {
        name: "cappuccino".to_string(),
        caffeine_per_ounce: 12.0,
    });
    all_coffee.push(BasicCoffee {
        name: "macchiato".to_string(),
        caffeine_per_ounce: 9.38,
    });
    all_coffee
}

// add custom coffee
pub fn add_coffee(new_coffee: BasicCoffee, all_coffee: &mut Vec<BasicCoffee>) -> &mut Vec<BasicCoffee> {
    all_coffee.push(new_coffee);
    all_coffee
}

// drink custom coffee
pub fn drink_custom(name: String, ounces: u32, caffeine_per_ounce: f32, history: &mut CaffeineHistory) -> Coffee {
    // add to caffeine history
    let new_coffee = Coffee {
        category: BasicCoffee {
            name: name.to_string(),
            caffeine_per_ounce: caffeine_per_ounce,
        },
        ounces: ounces,
    };
    // update total caffeine
    history.total_caffeine += caffeine_per_ounce * ounces as f32;
    println!("You have drunk {} oz of {} coffee", ounces, name);
    new_coffee
}