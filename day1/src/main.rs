use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day1_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.ends_with('\n') {
        contents.pop();
    }
    
    let l = contents.split('\n');

    for val in l {
        println!("Got: {}", val);
    }

    Ok(())
}
