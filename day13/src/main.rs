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

    // overwrite earlier variable, I like the name
    // we're working with modulos, so 'x' is now 1
    let bus_lines: Vec<u64> = f[1].split(',').map(|x| x.parse().unwrap_or(1)).collect();
    let mut sieve = bus_lines.clone();

    let mut ts: u64 = 0;
    let mut step = 1;

    loop {
        ts += step;

        let mut base_ts = ts;
        let mut modulos: Vec<u64> = Vec::new();
        for l in &bus_lines {
            let modulo = base_ts % *l;
            modulos.push(modulo);
            base_ts += 1;

            if modulo == 0 && sieve.contains(&l) {
                //println!("{} - {:?}", ts, modulos);
                step *= *l;
                sieve.retain(|x| x != l);
            }
        }
        //println!("{} - {:?}", ts, modulos);
        if modulos.iter().sum::<u64>() == 0 {
            break;
        }
        if ts == u64::MAX {
            println!("Not found");
            break;
        }
    }
    println!("Timestamp Pt. 2: {}", ts);
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
