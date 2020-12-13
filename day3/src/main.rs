use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day3_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    let mut tree_count = 0;
    let mut x = 0;

    let right_inc = 3;
    let down_inc = 1;

    let mut iter = contents.split('\n').step_by(down_inc);
    iter.next();
    
    for l in iter {
        // println!("X: {}, Line: {}", x, l);
        // println!("Char: {}", l.chars().nth(x).unwrap());
        x = (x + right_inc) % l.len();

        // unchecked unwrap is fine here cause we modulo'd x by the line length
        if l.chars().nth(x).unwrap() == '#' {
            tree_count+=1;
        }
    }

    println!("Tree count: {}", tree_count);
    Ok(())
}
