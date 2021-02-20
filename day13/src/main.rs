use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day13_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let f: Vec<&str> = contents.split('\n').collect();
    let ts: u64 = f[0].parse().unwrap();

    let bus_lines: Vec<u64> = f[1]
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse().unwrap())
        .collect();
    //println!("{} - {:?}", ts, bus_lines);

    let mut possible_departures: Vec<(u64, u64)> = bus_lines
        .iter()
        .map(|x| (closest_above(ts, *x), *x))
        .collect();
    possible_departures.sort_unstable();
    //println!("{:?}", possible_departures);
    let closest_departure = possible_departures[0];

    //println!("Earliest: {}", possible_departures[0].0);
    println!(
        "Result Pt. 1: {}",
        (closest_departure.0 - ts) * closest_departure.1
    );
    Ok(())
}

fn closest_above(ts: u64, line: u64) -> u64 {
    let mut closest = line;

    // brute force version
    while closest < ts {
        closest += line;
    }
    closest
}
