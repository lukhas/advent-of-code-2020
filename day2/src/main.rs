use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day2_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let str_contents = contents.trim();

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*)").unwrap();

    let valid_count1 = re.captures_iter(str_contents).filter(|c| is_policy1(&c)).count();
    let valid_count2 = re.captures_iter(str_contents).filter(|c| is_policy2(&c)).count();

    println!("Valid passwords Pt. 1: {}", valid_count1);
    println!("Valid passwords Pt. 2: {}", valid_count2);
    Ok(())
}

fn is_policy1(cap: &regex::Captures) -> bool {
    let lower:usize = cap.get(1).unwrap().as_str().parse().expect("Parse failure");
    let upper:usize = cap.get(2).unwrap().as_str().parse().expect("Parse failure");
    // it's actually just one character, trust me
    let letter:char = cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let pw:String = String::from(cap.get(4).unwrap().as_str());
    //println!("Lower: {} Upper: {} Letter: {}", lower, upper, letter);
    
    let letter_count:usize = pw.chars().filter(|c| c == &letter ).count();
    //println!("Letter {} found {} times in {}", letter, letter_count, pw);
    (lower <= letter_count) && (letter_count <= upper)
}

fn is_policy2(cap: &regex::Captures) -> bool {
    let p1:usize = cap.get(1).unwrap().as_str().parse().expect("Parse failure");
    let p2:usize = cap.get(2).unwrap().as_str().parse().expect("Parse failure");
    // it's actually just one character, trust me
    let letter:char = cap.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let pw:String = String::from(cap.get(4).unwrap().as_str());
    //println!("Lower: {} Upper: {} Letter: {}", lower, upper, letter);

    // safety first
    if (pw.chars().count() < p1) || (pw.chars().count() < p2) {
        return false;
    }

    (pw.chars().nth(p1-1).unwrap() == letter) ^ (pw.chars().nth(p2-1).unwrap() == letter)
}
