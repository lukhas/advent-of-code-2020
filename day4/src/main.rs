use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day4_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    println!("Valid passports: {}",contents.split("\n\n").filter(|p| is_valid_passport(p)).count());
    Ok(())
}

fn is_valid_passport(passport: &str) -> bool {
    let fields = [("byr:", 0b0000001),
                  ("iyr:", 0b0000010),
                  ("eyr:", 0b0000100),
                  ("hgt:", 0b0001000),
                  ("hcl:", 0b0010000),
                  ("ecl:", 0b0100000),
                  ("pid:", 0b1000000)];

    fields.iter().fold(0b0, |tot, field| {
        match passport.find(field.0) {
            None    => tot,
            Some(_) => tot + field.1,
        }
    }) == 0b1111111
}
