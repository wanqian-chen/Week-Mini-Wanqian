// Personal budget tracker

use std::io;

// define a struct to store details of a transaction
struct Transaction {
    trans_type: f32, // 1 for income, -1 for expense
    amount: f32,
    description: String,
}

// define a struct to represent a budget
struct Budget {
    income: f32,
    expenses: Vec<f32>,
    transactions: Vec<Transaction>,
}

// define a method to operate
impl Budget {
    fn new() -> Budget {
        Budget {
            income: 0.0,
            expenses: Vec::new(),
            transactions: Vec::new(),
        }
    }

    // define a method to add a transaction
    fn add(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    // show all transactions
    fn list(&self) {
        for (index, transaction) in self.transactions.iter().enumerate() {
            // if transaction is income, print income
            if (transaction.trans_type == 1.0) {
                println!("{}. Income: {}, Description: {}", index, transaction.amount, transaction.description);
            }
            else {
                println!("{}. Expense: {}, Description: {}", index, transaction.amount, transaction.description);
            }
        }
    }

    // show income details
    fn income(&self) {
        // get transactions with type 1
        let income_transactions = self.transactions.iter().filter(|t| t.trans_type == 1.0);
        // print income
        for (index, transaction) in income_transactions.enumerate() {
            println!("{}. {}, description: {}", index, transaction.amount, transaction.description);
        }
    }

    // show expense details
    fn expenses(&self) {
        // get transactions with type -1
        let expense_transactions = self.transactions.iter().filter(|t| t.trans_type == -1.0);
        // print expenses
        for (index, transaction) in expense_transactions.enumerate() {
            println!("{}. {}, description: {}", index, transaction.amount, transaction.description);
        }
    }

    // show balance
    fn balance(&self) {
        // get transactions with type 1
        let income_transactions = self.transactions.iter().filter(|t| t.trans_type == 1.0);
        // get transactions with type -1
        let expense_transactions = self.transactions.iter().filter(|t| t.trans_type == -1.0);
        // calculate income
        let income = income_transactions.fold(0.0, |sum, t| sum + t.amount);
        // calculate expenses
        let expenses = expense_transactions.fold(0.0, |sum, t| sum + t.amount);
        // calculate balance
        let balance = income - expenses;
        // print balance
        println!("Balance: {}", balance);
    }

    // delete a transaction
    fn delete_trans(&mut self, index: usize) {
        self.transactions.remove(index);
    }
}

// define a enum to represent a command
enum Command {
    Add { transaction: Transaction },
    List,
    Income,
    Expenses,
    Balance,
    Delete { index: usize },
    Quit,
}

// define a method to parse a command
impl Command {
    fn parse(input: &str) -> Command {
        let mut words = input.split_whitespace();
        
        match words.next() {
            Some("add") => Command::Add {
                transaction: Transaction {
                    trans_type: words.next().unwrap().parse().unwrap(),
                    amount: words.next().unwrap().parse().unwrap(),
                    description: words.collect::<Vec<&str>>().join(" "),
                },
            },
            Some("list") => Command::List,
            Some("income") => Command::Income,
            Some("expenses") => Command::Expenses,
            Some("balance") => Command::Balance,
            Some("delete") => Command::Delete {
                index: words.next().unwrap().parse().unwrap(),
            },
            Some("quit") => Command::Quit,
            _ => panic!("Unknown command"),
        }
    }
}

fn main() {
    // create a budget
    let mut budget = Budget::new();

    // welcome the user
    println!("Welcome to the budget tracker!");
    // print the menu
    println!("
    1. Add a transaction -> add + type(1 for income, -1 for expense) + amount + description
    2. Show all transactions -> list
    3. Show income details -> income
    4. Show expense details -> expenses
    5. Show balance -> balance
    6. Delete a transaction -> delete + index
    7. Quit -> quit
    ");

    loop {
        // read a command
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = Command::parse(&input);

        // execute the command
        match command {
            Command::Add { transaction } => budget.add(transaction),
            Command::List => budget.list(),
            Command::Income => budget.income(),
            Command::Expenses => budget.expenses(),
            Command::Balance => budget.balance(),
            Command::Delete { index } => budget.delete_trans(index),
            Command::Quit => break,
        }

        // success message
        println!("Command executed successfully! What else would you like to do?");
    }
}
