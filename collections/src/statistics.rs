use std::{collections::HashMap, io};

pub fn print_stats_from() {
    println!("Data like '1,2,3,6,7,-8' expected");

    let mut data = String::new();

    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read data to process");

    data = String::from(data.trim());
    
    println!("Processing {data} to extract median and mode");

    let mut data :Vec<i32> = data.split(",").map(|s| s.trim().parse().expect("Error parsing number")).collect();
    data.sort();

    let median: i32 = *data.get(data.len() / 2).expect("Error getting median");

    let mut data_agg: HashMap<i32,u32> = HashMap::new();
    for n in data {
        let entry = data_agg.entry(n).or_insert(0);
        *entry += 1;
    }

    let max_repetitions = data_agg.values().max().expect("Error calculating mode");

    let mode :i32 = find_key_from_value(&data_agg, max_repetitions).expect("Error extracting for mode");

   

    println!("Finished processing data.");
    println!("The median is {median}");
    println!("The mode is {mode}");

}

fn find_key_from_value(map: &HashMap<i32, u32>, value_to_find_by: &u32) -> Option<i32> {

    for (key, val) in map.iter() {
        match val == value_to_find_by {
            true => return Some(*key),
            false => continue,
        }
    }

    return None;
}