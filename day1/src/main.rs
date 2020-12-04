use std::fs::File;
use std::io::prelude::*;

use combination::combine::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day1_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    // make a u32 Vector out of the iter of strings
    let l1: Vec<u32> = contents.split('\n').map(|x| x.parse().unwrap()).collect();

    let val = combine_vec(&l1,2);
    for tuple in val {
        if tuple[0] + tuple[1] == 2020 {
            println!("Matching tuple: {:?}", tuple);
            println!("Solution: {}", tuple[0] * tuple[1]);
        }
    }

    Ok(())
}
