use std::io;

struct Order {
    name: String,
    price: f64,
    quantity: u32,
}

struct Orders {
    orders: Vec<Order>,
    total: f64,
}

impl Orders {
    fn new() -> Orders {
        Orders {
            orders: Vec::new(),
            total: 0.0,
        }
    }

    fn add(&mut self, name: String, price: f64, quantity: u32) {
        self.orders.push(Order {
            name,
            price,
            quantity,
        });
        self.total += price * quantity as f64;
    }

    fn list(&self) {
        for (index, order) in self.orders.iter().enumerate() {
            println!(
                "{}. {} {} {}",
                index, order.name, order.price, order.quantity
            );
        }
        println!("Total: {}", self.total);
    }
}

enum Command {
    Add { name: String, price: f64, quantity: u32 },
    List,
    Quit,
}

impl Command {
    fn parse(input: &str) -> Command {
        let mut words = input.split_whitespace();
        match words.next() {
            Some("add") => {
                let name = words.next().unwrap().to_string();
                let price = words.next().unwrap().parse().unwrap();
                let quantity = words.next().unwrap().parse().unwrap();
                Command::Add { name, price, quantity }
            }
            Some("list") => Command::List,
            Some("quit") => Command::Quit,
            _ => panic!("Unknown command"),
        }
    }
}

fn main() {
    let mut orders = Orders::new();
    // welcome message
    println!("Welcome to the order system!");
    loop {
        // print the menu
        println!("====================");
        println!("What would you like to do?");
        println!("add <name> <price> <quantity>");
        println!("list");
        println!("quit");
        println!("====================");
        
        // read the input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let command = Command::parse(input);
        match command {
            Command::Add { name, price, quantity } => orders.add(name, price, quantity),
            Command::List => orders.list(),
            Command::Quit => break,
        }
    }
}