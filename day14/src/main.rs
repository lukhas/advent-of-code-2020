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
    let mut mem_pt1: HashMap<usize, usize> = HashMap::new();
    let mut mem_pt2: HashMap<usize, usize> = HashMap::new();

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

            dec_to_bin(val, &mut bin);
            apply_mask(&mask, &mut bin);
            mem_pt1.insert(addr, bin_to_dec(&bin));

            // part 2, mask is applied on memory address
            dec_to_bin(addr, &mut bin);
            let addresses_list: Vec<String> = apply_mask_to_address(&mask, addr);
            for a in addresses_list {
                //println!("new addr: {} ({})", a, bin_to_dec(&a));
                mem_pt2.insert(bin_to_dec(&a), val);
            }
        }
    }
    let pt1: usize = mem_pt1.values().sum();
    println!("Pt. 1 result: {}", pt1);

    let pt2: usize = mem_pt2.values().sum();
    println!("Pt. 2 result: {}", pt2);
    Ok(())
}

// helper because that format string is unreadable
fn dec_to_bin(num: usize, bin: &mut String) {
    *bin = format!("{:036b}", num);
}

fn bin_to_dec(bin: &str) -> usize {
    usize::from_str_radix(bin, 2).unwrap()
}

fn apply_mask(mask: &str, bin: &mut String) {
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

fn apply_mask_to_address(mask: &str, addr: usize) -> Vec<String> {
    let mut addr_bin = String::new();
    dec_to_bin(addr, &mut addr_bin);

    let mut res: Vec<String> = vec!["".to_string(); 1];

    for bit in mask.chars().zip(addr_bin.chars()) {
        match bit.0 {
            'X' => {
                let mut forked_addresses: Vec<String> = Vec::new();
                for address in res.iter_mut() {
                    let mut forked_address = address.clone();
                    address.push('0');
                    forked_address.push('1');
                    forked_addresses.push(forked_address);
                }
                res.append(&mut forked_addresses);
            }

            '1' => {
                for address in res.iter_mut() {
                    address.push('1');
                }
            }
            _ => {
                for address in res.iter_mut() {
                    address.push(bit.1);
                }
            }
        };
    }

    res
}
