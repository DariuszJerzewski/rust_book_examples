use std::io;

fn main() {
    let x = get_value();

    if x == 0 {
        println!("The user provided an invalid integer!");
    } else {
        println!("The user provided an integer with a value: {x}!");
    }

    let ternary = if x > 0 { 42 } else { 666 }; // if an "if" arm returns a value, all other arms must return a value of the same type!

    println!("The ternary operator has spoken! The value is {ternary}!");

    let mut counter = 0;
    let x = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The fancy variable value is {x}!");

    while counter > 0 {
        println!("Counting down {counter}");
        counter -= 1;
    }

    let arr = [3; 5];

    for value in arr {
        println!("My loop is saying {value}!");
    }

    for numver in (1..4).rev() {
        println!("Counting down {numver}");
    }
}

fn get_value() -> i32 {
    loop {
        println!("Provide a valid integer:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something has gone really wrong here!");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        }
    }
}
