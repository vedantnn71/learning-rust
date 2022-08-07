use std::io;

fn main() {
    let mut to = String::new();
    let mut value = String::new();

    log();

    io::stdin()
        .read_line(&mut to)
        .expect("Failed to read the input");

    println!("What's the value?");
    
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read the input");

    let to: isize = to.trim().parse().expect("Please provide an integer");
    let value: isize = value.trim().parse().expect("Please provide a number");

    if to == 1 {
        println!("{value} Celsius to farhenite = {}F", to_farhenite(value));
    } else if to == 2 {
        println!("{value} Fahrenheit to celsius = {}C", to_celsius(value));
    }
}

fn to_farhenite(celsius: isize) -> isize {
    return (celsius * 9/5) + 32;
} 

fn to_celsius(farhenite: isize) -> isize {
    return (farhenite - 32) * 5/9;
} 

fn log() {
    println!("##### Convert temperatures between Fahrenheit and Celsius #####");
    println!("##### In RUST! #####\n");
    println!("Choose (in number)");
    println!("[1] - Celsius to farhenite");
    println!("[2] - Farhenite to celsius");
}
