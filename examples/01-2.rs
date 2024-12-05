use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("./inputs/01.txt")?;
    println!("{}", input);
    Ok(())
}
