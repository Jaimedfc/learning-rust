use std::io;

fn main() { 
    println!("Which fibonacci number do you want to calculate?");

    let mut fibo = String::new();

    io::stdin()
        .read_line(&mut fibo)
        .expect("Failed to read line");

    let fibo: u32 = match fibo.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            return;
        }
    };

    println!("The {}th fibonacci number is: {}", fibo, fibonacci(fibo));

}

fn fibonacci(n :u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fibonacci(n - 1) + fibonacci(n - 2);
}
