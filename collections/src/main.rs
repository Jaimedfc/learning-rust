use std::{collections::HashMap, i32, io};

fn main() {
    println!("Hi, which problem would you like to resolve?");
    println!("- 1: Get median and mode from a list of numbers.");
    println!("- 2: Traslate a word to Pig Latin");
    println!("- 3: Manage employees of different departments");

    let mut feat = String::new();

    io::stdin()
        .read_line(&mut feat)
        .expect("Failed to read line");


    let feature = match feat.trim() {
        "1" => Feature::Stats,
        "2" => Feature::Translator,
        "3" => Feature::Management,
        _ => {
            println!("Please, select a valid feature. 1, 2 or 3");
            return;
        }
    };


    println!("{feature:?} selected! Please introduce the data to process");
    feature.init_data();

    let mut data = String::new();

    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read data to process");

    feature.process(&data);

    println!("Processing ended!!")
}


#[derive(Debug)]
enum Feature {
    Stats,
    Translator,
    Management
}

impl Feature {
    fn init_data(&self) {
        match self {
            Feature::Stats => println!("Data like '1,2,3,6,7,-8' expected"),
            Feature::Translator => println!("Data like 'hello' or 'world' expected"),
            Feature::Management => println!("Data like 'Add <name> to <department>' or 'Remove <name> from <department>'")
        }
        
    }

    fn process(&self, data: &String) {
        match self {
            Feature::Stats => {
                println!("Processing {data} to extract median and mode");
                let mut data :Vec<i32> = data.split(",").map(|s| s.trim().parse().expect("Error parsing number")).collect();
                data.sort();
                let median = data.get(data.len() / 2).expect("Error getting median");
                println!("The median is {median}");
                let mut res: HashMap<i32,u32> = HashMap::new();
                for n in data {
                    let entry = res.entry(n).or_insert(0);
                    *entry += 1;
                }

                let mode_value = res.values().max().expect("Error calculating mode");

                for (key,value) in &res {
                    if value == mode_value {
                        println!("The mode is {key}");
                        return;
                    }
                }
            },
            Feature::Translator => println!("Processing Translator"),
            Feature::Management => println!("Processing Management")
        }
    }
}
