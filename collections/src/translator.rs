use std::io;

pub fn translate_to_pig_latin() {
    println!("Data like 'hello' or 'world' expected");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read data to process");
    
    word = String::from(word.trim().to_lowercase());

    if word == "" {
        println!("Please provide a non-empty word!");
        return;
    }

    println!("Translating {word} to Pig Latin");

    let mut chars :Vec<char>= word.chars().collect();

    let translation :String;

    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];


    if VOWELS.contains(chars.get(0).expect("Error getting 1st character from word")) {
        translation = word.clone() + "-hay";
    } else {
        let word_without_first_char = chars.split_off(1);

        let word_without_first_char :String = word_without_first_char.iter().collect();

        translation = word_without_first_char + "-" + &chars[0].to_string() + "ay";
    }


    println!("The word {word} is tranlated to {translation}");
}