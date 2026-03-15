use std::{collections::HashMap, io};

pub fn run_company() {
    println!("Data like 'Add <name> to <department>' or 'Remove <name> from <department>'");

    let mut status = Company {
        status: HashMap::new()
    };

    loop {
        
        let mut data = String::new();

        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read data to process");

        data = String::from(data.trim());

        if data.to_lowercase() == "exit" {
            println!("Company management programn ending...");
            println!("Company last status: {status:?}");
            return;
        }

        let tokens :Vec<String> = data.split_whitespace().map(String::from).collect();

        if tokens.len() != 4 {
            println!("Invalid pattern found in data to process");
            println!("Data like 'Add <name> to <department>', 'Remove <name> from <department>' or 'exit' is expected");
            continue;
        }

        let op :Operation = match tokens[0].to_lowercase().as_str() {
            "add" => Operation::Addition,
            "remove" => Operation::Subtraction,
            _ => {
                println!("Found invalid Operation: {data}");
                println!("'Add' or 'Remove' expected");
                continue;
            }
        };

        let department = &tokens[3];
        let employee = &tokens[1];

        match op {
            Operation::Addition => status.add_employee(department, employee),
            Operation::Subtraction => status.remove_employee(department, employee),
        }

        println!("Final Company Status: {status:?}");
        println!("Enter new data to process or type 'exit' to finish");

    }

}

enum Operation {
    Addition,
    Subtraction
}

impl Company {

    fn add_employee(&mut self, department: &String, employee: &String) {
        let employees = self.status.entry(department.to_lowercase().clone()).or_insert(Vec::new());
        employees.push(employee.clone());
    }

    fn remove_employee(&mut self, department: &String, employee: &String) {
        let employees :&mut Vec<String> = self.status.entry(department.to_lowercase().clone()).or_insert(Vec::new());
        *employees = employees.iter().filter(|item| item.to_lowercase() != employee.to_lowercase()).map(String::clone).collect();
    }
    
}

#[derive(Debug)]
struct Company {
    status: HashMap<String, Vec<String>>
}