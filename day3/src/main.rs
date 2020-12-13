use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day3_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    println!("Tree count pt1: {}", toboggan_walk(&contents, 3, 1));
    println!("Result pt2: {}",
             toboggan_walk(&contents, 1, 1) *
             toboggan_walk(&contents, 3, 1) *
             toboggan_walk(&contents, 5, 1) *
             toboggan_walk(&contents, 7, 1) *
             toboggan_walk(&contents, 1, 2)
    );
    Ok(())
}

fn toboggan_walk(contents: &String, right_step: usize, down_step: usize) -> usize {
    let mut x = 0;
    let mut count = 0;

    let mut iter = contents.split('\n').step_by(down_step);
    iter.next();
    
    for l in iter {
        // println!("X: {}, Line: {}", x, l);
        // println!("Char: {}", l.chars().nth(x).unwrap());
        x = (x + right_step) % l.len();

        // unchecked unwrap is fine here cause we modulo'd x by the line length
        if l.chars().nth(x).unwrap() == '#' {
            count+=1;
        }
    }
    
    count
}
