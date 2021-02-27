use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day14_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut mask = String::new();
    let mut bin = String::new();
    let mut mem: HashMap<usize, usize> = HashMap::new();

    lazy_static! {
        static ref RE_MASK: Regex = Regex::new(r"^mask\s+=\s+(.*)$").unwrap();
        static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\]\s+=\s+(\d+)$").unwrap();
    }

    for line in contents.split('\n') {
        if let Some(x) = RE_MASK.captures(line) {
            mask = x.get(1).unwrap().as_str().to_string();
            //println!("mask: {}", mask);
        }

        if let Some(x) = RE_MEM.captures(line) {
            let addr: usize = x.get(1).unwrap().as_str().parse().unwrap();
            let val: usize = x.get(2).unwrap().as_str().parse().unwrap();
            //println!("addr: {} - val: {}", addr, val);

            dec_to_bin(val, &mut bin);
            //println!("binary for {:10} is {}", val, bin);

            apply_mask(&mask, &mut bin);
            //println!("result for {:10} is {} ({})", val, bin, bin_to_dec(&bin));

            mem.insert(addr, bin_to_dec(&bin));
        }
    }
    let pt1: usize = mem.values().sum();
    println!("Pt. 1 result: {}", pt1);
    Ok(())
}

// helper because that format string is unreadable
fn dec_to_bin(num: usize, bin: &mut String) {
    *bin = format!("{:036b}", num);
}

fn bin_to_dec(bin: &String) -> usize {
    usize::from_str_radix(bin, 2).unwrap()
}

fn apply_mask(mask: &String, bin: &mut String) {
    let mut new2 = String::new();
    for bit in mask.chars().zip(bin.chars()) {
        match bit.0 {
            '0' => new2.push('0'),
            '1' => new2.push('1'),
            _ => new2.push(bit.1),
        }
    }

    *bin = new2;
}
