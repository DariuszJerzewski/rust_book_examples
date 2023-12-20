#[derive(Debug, PartialEq)]
pub enum ExampleErrors {
    InvalidValue,
}

pub fn unrecoverable_error(test: &str) {
    if test == "break" {
        panic!("It's completely broken?!?!");
    }
}

pub fn recoverable_error(test: &str) -> Result<(), ExampleErrors> {
    if test == "break" {
        return Err(ExampleErrors::InvalidValue);
    }

    Ok(())
}

pub fn propagating_errors() -> Result<(), ExampleErrors> {
    match recoverable_error("break") {
        Err(error) => {
            println!("Caught {error:?}");

            Err(error)
        }

        Ok(result) => {
            println!("Invoked method has succeeded.");

            Ok(result)
        }
    }
}

pub fn simplified_propagation() -> Result<(), ExampleErrors> {
    recoverable_error("break")?;

    Ok(())
}
