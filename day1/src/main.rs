
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day1_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    // make a u32 Vector out of the iter of strings
    let number_list: Vec<u32> = contents.split('\n').map(|x| x.parse().expect("Input error")).collect();

    for a in &number_list {
        for b in &number_list {
            //println!("a,b: {} - {}",a, b);
            if a + b == 2020 {
                println!("Matching tuple pt. 1: {} {}", a, b);
                println!("Solution pt. 1: {}", a * b);
            }
            for c in &number_list {
                if a + b + c == 2020 {
                    println!("Matching tuple pt. 2: {} {} {}", a, b, c);
                    println!("Solution pt. 2: {}", a * b * c);
                }
            }
        }
    }
    
    Ok(())
}
