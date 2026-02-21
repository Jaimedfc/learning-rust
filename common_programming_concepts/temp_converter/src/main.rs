use std::io;

fn main() {
    
    println!("Welcome to the Temperature Converter!\n");

    let mut temp = String::new();
    let mut temp_type = String::new();

    println!("Enter a temperature to convert: ");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp :f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature value. Please enter a valid number.");
            return;
        }
    };

    println!("Is this temperature in Celsius or Fahrenheit? (C/F): ");

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line");

    let temp_type = temp_type.trim().to_uppercase();
    if ![String::from("C"), String::from("F")].contains(&temp_type) {
        println!("Invalid temperature type. Please enter 'C' for Celsius or 'F' for Fahrenheit.");
        return;
    }

    let temp_type = match temp_type.as_str() {
        "C" => "Celsius",
        "F" => "Fahrenheit",
        _ => unreachable!(),
    };

    println!("Converting {} degrees {}...", temp, temp_type);

    let converted_temp = match temp_type == "Celsius" {
        true => (temp * 9.0 / 5.0) + 32.0,
        false => (temp - 32.0) * 5.0 / 9.0,
    };

    let converted_temp_type = match temp_type == "Celsius" {
        true => "Fahrenheit",
        false => "Celsius",
    };
    
    println!("{} degrees {} is {} degrees {}", temp, temp_type, converted_temp, converted_temp_type);
}
