use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day5_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    // assume input is properly formatted
    let mut ids = contents.split('\n').map(|line| binary_to_row(line.get(0..7).unwrap()) * 8 + binary_to_column(line.get(7..10).unwrap())).collect::<Vec<_>>();
    ids.sort();

    println!("Highest ID: {}", ids.last().unwrap());
    println!("Free seat: {}", find_gap(ids));
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

fn find_gap(list: Vec<u32>) -> u32 {
    list.windows(2).fold(0, |seat, elem | {
        let a = elem.get(0).unwrap();
        let b = elem.get(1).unwrap();

        if *b - 1 != *a {
            *b - 1
        } else {
            seat
        }
    })
}
