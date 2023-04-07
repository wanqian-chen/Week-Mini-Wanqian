use std::io;

struct TodoList {
    things: String,
    date: String
}

struct TodoLists {
    tasks: Vec<TodoList>,
}

// a function to get date in the format of yyyy-mm-dd
fn get_date() -> String {
    let mut date = String::new();
    let now = chrono::Local::now();
    date.push_str(&now.format("%Y-%m-%d").to_string());
    date
}

impl TodoLists {
    fn new() -> TodoLists {
        TodoLists { tasks: Vec::new() }
    }

    fn add(&mut self, things: String, date: String) {
        self.tasks.push(TodoList { things, date });
    }

    fn done(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    fn list(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {} {}. ", index, task.things, task.date);
        }
    }

    // get today's list
    fn today(&self) {
        let date = get_date();
        for (index, task) in self.tasks.iter().enumerate() {
            if task.date == date {
                println!("{}. {} {}. ", index, task.things, task.date);
            }
        }
    }
}

enum Command {
    Add { things: String, date: String },
    Done { index: usize },
    List,
    Today,
    Quit,
}

impl Command {
    fn parse(input: &str) -> Command {
        let mut words = input.split_whitespace();
            
        match words.next() {
            Some("add") => Command::Add {
                things: words.next().unwrap().to_string(),
                date: words.next().unwrap().to_string()
            },
            Some("done") => Command::Done {
                index: words.next().unwrap().parse().unwrap(),
            },
            Some("list") => Command::List,
            Some("today") => Command::Today,
            Some("quit") => Command::Quit,
            _ => panic!("Unknown command"),
        }
        }
    
}

fn main() {
    let mut todo_lists = TodoLists::new();

    // print the menu
    println!("What would you like to do?");

    // a loop to get user's input and execute the command
    loop {
        println!("1. Add a task -> add + task + date(yyyy-mm-dd)");
        println!("2. Mark a task as done -> done + index");
        println!("3. List all tasks -> list");
        println!("4. List today's tasks -> today");
        println!("5. Quit -> quit");
        println!("==============================");

        // get user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // parse the input
        let command = Command::parse(&input);

        // execute the command
        match command {
            Command::Add { things, date } => todo_lists.add(things, date),
            Command::Done { index } => todo_lists.done(index),
            Command::List => todo_lists.list(),
            Command::Today => todo_lists.today(),
            Command::Quit => break,
        }

        println!("==============================");
        println!("What would you like to do next?");
    }

}


