use std::io;

fn main() {
    println!("This is my main function!");

    another_function();

    parameterised_function(5);

    print_temperature(32.5, 'C');

    let x = get_value(0);

    println!("The value obtained from user is {x}!");
}

fn another_function() {
    println!("This is another function!");
}

fn parameterised_function(x: i32) {
    println!("The argument passed to x parameter is: {x}!");
}

fn print_temperature(temperature: f64, unit: char) {
    println!("The temperature is: {temperature}*{unit}!");
}

fn get_value(default: i32) -> i32 {
    println!("Provide a valid integer:");
    
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input value!");

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => default,
    }
}
