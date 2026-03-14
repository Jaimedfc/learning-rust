use std::io;

use collections::{com_management, statistics, translator};

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
    feature.process();

    println!("Processing ended!!")
}


#[derive(Debug)]
enum Feature {
    Stats,
    Translator,
    Management
}

impl Feature {
    fn process(&self) {
        match self {
            Feature::Stats => statistics::print_stats_from(),
            Feature::Translator => translator::translate_to_pig_latin(),
            Feature::Management => com_management::run_company()
        }
    }
}
