use std::io;

// define a struct to represent a todo list
struct TodoList {
    tasks: Vec<String>,
}

// define a method to create a new todo list
impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    // define a method to add a task to the todo list
    fn add(&mut self, task: String) {
        self.tasks.push(task);
    }

    // define a method to mark a task as done
    fn done(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    // define a method to list all tasks
    fn list(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", index, task);
        }
    }
}

// define a enum to represent a command
enum Command {
    Add { task: String },
    Done { index: usize },
    List,
    Quit,
}

// define a method to parse a command
impl Command {
    fn parse(input: &str) -> Command {
        let mut words = input.split_whitespace();
        
        match words.next() {
            Some("add") => Command::Add {
                task: words.collect::<Vec<&str>>().join(" "),
            },
            Some("done") => Command::Done {
                index: words.next().unwrap().parse().unwrap(),
            },
            Some("list") => Command::List,
            Some("quit") => Command::Quit,
            _ => panic!("Unknown command"),
        }
    }
}

fn main() {
    // create a todo list
    let mut todo_list = TodoList::new();

    // welcome the user to the todo list
    println!("Welcome to the todo list!");
    // print the menu
    println!("
    1. Add a task -> add + task
    2. Mark a task as done -> done + index
    3. List all tasks -> list
    4. Quit -> quit
    ");

    // a loop to get user's input and execute the command
    loop {
        // get user's input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // parse the input
        let command = Command::parse(&input);

        // execute the command
        match command {
            Command::Add { task } => todo_list.add(task),
            Command::Done { index } => todo_list.done(index),
            Command::List => todo_list.list(),
            Command::Quit => break,
        }

        // print indicating a successful execution
        println!("Command executed successfully!");
    }

}