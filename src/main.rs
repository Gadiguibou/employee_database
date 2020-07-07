use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match Command::from_input(input) {
            Some(Command::Add { name, department }) => employees
                .entry(department)
                .or_default()
                .push(name),
            Some(Command::List(department)) => println!("{:?}", employees.entry(department)),
            Some(Command::ListAll) => {
                for department in &employees {
                    println!("{:?}", department);
                }
            }
            Some(Command::Quit) => break,
            None => println!("No command found with this name"),
        }
    }

    println!("Exiting program, all data is now cleared.");
}

enum Command {
    Add { name: String, department: String },
    List(String),
    ListAll,
    Quit,
}

impl Command {
    fn from_input(s: &str) -> Option<Self> {
        let split: Vec<&str> = s.trim().split_whitespace().collect();

        match split.as_slice() {
            ["Add", name, "to", department] => Some(Command::Add {
                name: name.to_string(),
                department: department.to_string(),
            }),
            ["List", "all"] => Some(Command::ListAll),
            ["List", department] => Some(Command::List(department.to_string())),
            ["Quit"] => Some(Command::Quit),
            _ => None,
        }
    }
}
