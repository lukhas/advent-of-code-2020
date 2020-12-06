#![feature(bool_to_option)]

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day2_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    // match
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): (.*)$").unwrap();

    let mut valid_count = 0;
    
    for line in contents.split('\n') {
        for cap in re.captures_iter(line) {
            let lower:i8 = cap[1].parse().expect("Parse failure");
            let upper:i8 = cap[2].parse().expect("Parse failure");
            // it's actually just one character, trust me
            let letter:char = cap[3].chars().nth(0).unwrap();
            let pw:String = String::from(&cap[4]);
            // println!("Lower: {} Upper: {} Letter: {}", lower, upper, letter);

            let count = pw.chars().fold(0, | count, c| -> i8 {
                if c == letter {
                    count + 1
                } else {
                    count
                }
            });
            //println!("Letter {} found {} times in {}", letter, count, pw);
            if (lower <= count) && (count <= upper) {
                valid_count = valid_count + 1;
            }
        }
    }

    println!("Valid passwords: {}", valid_count);
    Ok(())
}
