use std::fs::File;
use std::io::prelude::*;

#[macro_use] extern crate lazy_static;
use regex::Regex;

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
    // println!("");
    // println!("{}",passport);

    has_valid_byr(passport) &&
        has_valid_iyr(passport) &&
        has_valid_eyr(passport) &&
        has_valid_hgt(passport) &&
        has_valid_hcl(passport) &&
        has_valid_ecl(passport) &&
        has_valid_pid(passport)
}

fn has_valid_byr(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"byr:([0-9]{4})").unwrap();
    }

    match RE.captures(passport) {
        None    => false,
        Some(x) => {
            let year = number_from_match(&x);
            (year >= 1920) && (year <= 2002)
        }
    }
}

fn has_valid_iyr(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"iyr:([0-9]{4})").unwrap();
    }

    match RE.captures(passport) {
        None    => false,
        Some(x) => {
            let year = number_from_match(&x);
            (year >= 2010) && (year <= 2020)
        }
    }
}

fn has_valid_eyr(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"eyr:([0-9]{4})").unwrap();
    }

    match RE.captures(passport) {
        None    => false,
        Some(x) => {
            let year = number_from_match(&x);
            (year >= 2020) && (year <= 2030)
        }
    }
}

fn has_valid_hgt(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"hgt:([0-9]+)(in|cm)").unwrap();
    }

    match RE.captures(passport) {
        None    => false,
        Some(x) => {
            let hgt =  number_from_match(&x);
            let unit = x.get(2).unwrap().as_str();
            (unit == "cm" && hgt >= 150 && hgt <= 193) ||
                (unit == "in" && hgt >= 59 && hgt <= 76)
        }
    }
}

fn has_valid_hcl(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"hcl:#[0-9a-f]{6}").unwrap();
    }

    match RE.captures(passport) {
        None    => false,
        Some(_) => true
    }
}

fn has_valid_ecl(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap();
    }

    match RE.captures(passport) {
        None    => false,
        Some(_) => true
    }
}

fn has_valid_pid(passport: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"pid:([0-9]{9})").unwrap();
    }

    match RE.captures(passport) {
        None    => false,
        Some(_) => true
    }
}

fn number_from_match(field: &regex::Captures) -> u32 {
    field.get(1).unwrap().as_str().parse().unwrap()
}
