use std::fs::File;
use std::io::prelude::*;

use combination::combine::*;
use std::collections::{HashSet};

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day9_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    let v: Vec<u128> = contents.split('\n').map(|x| x.parse().expect("Input error")).collect();

    let weakness = get_weakness(&v,25);
    println!("Weakness (pt. 1): {}", weakness);

    let subslice = get_subslice(&v, weakness);
    //println!("Matching subslice {:?}", subslice);
    let min = subslice.iter().min().unwrap();
    let max = subslice.iter().max().unwrap();
    println!("Weakness (pt. 2): {}", min + max);
    Ok(())
}

fn get_weakness(v: &Vec<u128>, preamble_length: usize) -> u128 {
    let mut start = preamble_length;

    while start < v.len() {
        let preamble = &v[start-preamble_length..start];
        let combined = combine_vec(&Vec::from(preamble), 2);
        let curr = v[start];

        let mut h: HashSet<u128> = HashSet::new();
        for elem in combined.iter() {
            h.insert(elem[0] + elem[1]);
        }

        if !h.contains(&curr) {
            return curr
        }
        start+=1;
    }
    // return 0 if we arrived here
    0
}

fn get_subslice(v: &Vec<u128>, weakness: u128) -> &[u128] {
    let mut start = 0;
    while start < v.len() -1 {
        let slice = &v[start..];
        let mut upper_end = 2;

        while upper_end <= slice.len() {
            let subslice = &slice[0..upper_end];
            let sum: u128 = subslice.iter().sum();
            if sum == weakness {
                return &subslice
            }
            //println!("sub: {:?}", subslice);
            upper_end+=1;
        }
        start += 1;
    };
    // return empty if we arrived here
    &[]
}
