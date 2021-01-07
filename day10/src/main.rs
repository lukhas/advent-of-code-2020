use std::fs::File;
use std::io::prelude::*;

use std::collections::{HashSet,HashMap};

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day10_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    let mut h: HashSet<i64> = contents.split('\n').map(|x| x.parse().expect("Input error")).collect();
    let mut differences: HashMap<i64,i64> = HashMap::new();

    let mut cur_jolt = 0;
    
    while !h.is_empty() {
        let min = get_min(&h);

        //println!("curr: {} / min: {:?}", cur_jolt, min);
        let diff = (min - cur_jolt).abs();
        //println!("Difference with current joltage: {}", diff);
        cur_jolt = min;
        h.remove(&min);

        differences_inc(&mut differences, diff);
    }
    // Finally, always a difference of 3 with our device
    differences_inc(&mut differences, 3);

    println!("Pt. 1: {}", differences[&1] * differences[&3]);

    // part 2
    let mut v: Vec<i64> = contents.split('\n').map(|x| x.parse().expect("Input error")).collect();
    v.push(0);
    v.sort();
    let mut paths: Vec<i64> = vec![0; v.len()];
    paths[0] = 1;

    for elem in v.iter().enumerate() {
        let idx     = elem.0;

        store_gap(&v, &mut paths, idx, 1);
        store_gap(&v, &mut paths, idx, 2);
        store_gap(&v, &mut paths, idx, 3);
    }

    //println!("{:?}\n{:?}",v,paths);
    println!("Pt. 2: {}",paths.iter().last().unwrap());
    Ok(())
}

fn get_min(h: &HashSet<i64>) -> i64 {
    *h.iter().min().unwrap()
}

fn differences_inc(differences: &mut HashMap<i64,i64>, diff: i64) {
    let entry = differences.entry(diff).or_insert(0);
    *entry += 1;
}

fn store_gap(v: &Vec<i64>, paths: &mut Vec<i64>, idx: usize, gap: usize) {
    if v.get(idx + gap).is_some() {
        let current   = v[idx];
        let neighbour = v[idx + gap];
        if neighbour - current <= 3 {
            let path = paths[idx];
            let x = &mut paths[idx + gap];
            *x += path;
        }
    }
}
