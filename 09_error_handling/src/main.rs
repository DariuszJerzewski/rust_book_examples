mod panic;

struct Test {
    tys: String,
}

fn handle_example_error(error: panic::ExampleErrors) {
    match error {
        panic::ExampleErrors::InvalidValue => {
            println!("The function has returned an error message {error:?}");
        }
    }
}

fn main() -> Result<(), panic::ExampleErrors> {
    // This main declaration allows use of ? operator in main
    println!("Hello, world!");

    // adding panic = 'abort'
    // panic::unrecoverable_error("break"); // it will panic when it is uncommented

    let v = vec![1, 2, 3];

    // v[99]; // this will panic in 3rd party code

    match panic::recoverable_error("break") {
        Err(error) => handle_example_error(error),
        Ok(_) => println!("The function has returned successfully!"),
    }

    panic::recoverable_error("test").unwrap_or_else(|error| {
        if error == panic::ExampleErrors::InvalidValue {
            println!("The function has returned an error message {error:?}");
        } else {
            println!("The function has returned successfully!");
        }
    });

    // panic::recoverable_error("break").unwrap(); // if error then panic

    // panic::recoverable_error("break")
    //     .expect("This is a custom message that will be displayed when the program panics.");

    panic::propagating_errors().unwrap_or_else(|_| {
        println!("The function propagated the original error");
    });

    panic::simplified_propagation().unwrap_or_else(|_| {
        println!("The error was propagated without a match statement");
    });

    panic::recoverable_error("break")?;

    Ok(())
}
