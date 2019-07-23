use std::io;
use std::collections::HashMap;

fn main() {

    let mut employees = HashMap::new();

    loop {
        println!("Add a person to the companies department!");
        println!("Input their name or type 'quit' to exit:");

        let mut name = String::new();

        io::stdin().read_line(&mut name)
            .expect("Failed to read line");

        println!("Input their department:");

        let mut department = String::new();

        io::stdin().read_line(&mut department)
            .expect("Failed to read line");

        employees.insert(name, department);

        println!("Here are all the employees: {:#?}", employees)
    }
}
