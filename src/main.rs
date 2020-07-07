use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    println!("\n---------------");

    println!("\nWelcome to the employee database!\n");

    println!("Type 'Add <name> to <department>' to add an employee.");
    println!("Type 'Remove <name> from <department>' to remove an employee.");
    println!("Type 'Remove <department>' to remove a department with all its employees.");
    println!("Type 'List <department>' to list all employees in the department.");
    println!("Type 'List all' to list all employees in the database.");
    println!("Type 'Quit' to quit.\n");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match Command::from_input(input) {
            Some(Command::Add { name, department }) => {
                employees.entry(department).or_default().push(name)
            }
            Some(Command::Remove { name, department }) => match employees.get_mut(&department) {
                Some(names) => {
                    match names.iter().position(|x| *x == *name) {
                        Some(x) => {
                            names.remove(x);
                        }
                        None => println!(
                            "No employee with name '{}' found in department '{}'",
                            name, department
                        ),
                    };
                }
                None => println!("No department found with this name."),
            },
            Some(Command::RemoveDept(department)) => match employees.remove(&department) {
                Some(_) => println!("Removed department '{}' and all its employees", department),
                None => println!("No department found with this name"),
            }
            Some(Command::List(department)) => match employees.get_mut(&department) {
                Some(employees) => {
                    println!("---------------");
                    println!("Employees in department '{}':", department);
                    employees.sort();
                    for name in employees {
                        println!("- {}", name);
                    }
                    println!("---------------");
                }
                None => println!("No department found with name '{}'", department),
            },
            Some(Command::ListAll) => {
                println!("~~~~~~~~~~~~~~~");
                println!("ALL EMPLOYEES IN DATABASE:");
                println!("---------------");
                for (department, names) in &mut employees {
                    println!("Employees in department '{}':", department);
                    names.sort();
                    for name in names {
                        println!(" - {}", name);
                    }
                    println!("---------------");
                }
                println!("~~~~~~~~~~~~~~~");
            }
            Some(Command::Quit) => break,
            None => println!("No command found with this name"),
        }
    }

    println!("Exiting program, all data is now cleared.");
    println!("---------------");
}

enum Command {
    Add { name: String, department: String },
    Remove { name: String, department: String },
    RemoveDept(String),
    ListAll,
    List(String),
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
            ["Remove", name, "from", department] => Some(Command::Remove {
                name: name.to_string(),
                department: department.to_string(),
            }),
            ["Remove", "department", department] => Some(Command::RemoveDept(department.to_string())),
            ["List", "all"] => Some(Command::ListAll),
            ["List", department] => Some(Command::List(department.to_string())),
            ["Quit"] => Some(Command::Quit),
            _ => None,
        }
    }
}
