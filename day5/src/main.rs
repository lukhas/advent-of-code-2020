use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day5_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    // assume input is properly formatted
    let high = contents.split('\n').fold(0,|high, line| {
        let id = binary_to_row(line.get(0..7).unwrap()) * 8 + binary_to_column(line.get(7..10).unwrap());
        //println!("id: {}", id);
        if id > high {
            id }
        else {
            high
        }
    });

    println!("Higest ID: {}", high);
    
    Ok(())
}

fn binary_to_row(line: &str) -> u32 {
    binary_search(line, 0, 127, 'F', 'B')
}

fn binary_to_column(line: &str) -> u32 {
    binary_search(line, 0, 7, 'L', 'R')
}

fn binary_search(s: &str, mut lower: u32, mut upper: u32, l_code: char, u_code: char) -> u32 {
    let mut range = upper - lower + 1; // + 1 because they start numbering at 0

    for c in s.chars() {
        range = range / 2;
        if c == l_code {
            upper = upper - range;
        }
        if c == u_code {
            lower = lower + range;
        }
    }
    lower
}
