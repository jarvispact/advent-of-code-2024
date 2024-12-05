use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input: String = fs::read_to_string("./inputs/1.txt")?;
    let lines = input.lines();

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in lines {
        let mut parts = line.split("   ");

        let one = parts
            .nth(0)
            .and_then(|val| Some(val.parse::<u32>().unwrap()));

        let two = parts
            .nth(0)
            .and_then(|val| Some(val.parse::<u32>().unwrap()));

        if let Some(x) = one {
            list1.push(x);
        }

        if let Some(x) = two {
            list2.push(x);
        }
    }

    list1.sort();
    list2.sort();

    let mut answer = 0;

    for (one, two) in list1.iter().zip(list2.iter()) {
        let distance = one.abs_diff(*two);
        answer += distance;
    }

    println!("Answer: {}", answer);

    Ok(())
}
