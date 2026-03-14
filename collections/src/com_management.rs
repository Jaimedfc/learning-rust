use std::{collections::HashMap, io};

fn manage_company(records: &HashMap<String, Vec<String>>, data: &String) -> HashMap<String, Vec<String>> {
    println!("Processing {data} to manage my company");

    return HashMap::new()
}

pub fn run_company() {
    println!("Data like 'Add <name> to <department>' or 'Remove <name> from <department>'");

    let mut data = String::new();

    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read data to process");

    data = String::from(data.trim());

}