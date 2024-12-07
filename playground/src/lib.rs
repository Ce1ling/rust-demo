use std::error;

pub fn run() -> Result<(), Box<dyn error::Error>> {
    println!("Hello world!");

    Ok(())
}
