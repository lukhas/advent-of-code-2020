use std::io::prelude::*;
use std::{fs::File, usize};

use std::collections::HashMap;

type NDimensions = Vec<isize>;
#[derive(Clone)]
struct InfiniteCube {
    dimensions: i32,
    cubes: HashMap<NDimensions, bool>,
}

// assume this is enough for infinity
static OFFSET: isize = 15;

use itertools::Itertools;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day17_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Assume this is infinite size for the purpose of this exercise
    let mut universe = InfiniteCube {
        dimensions: 3,
        cubes: HashMap::new(),
    };

    // Initial state
    let (mut x, mut y, z) = (OFFSET, OFFSET, OFFSET);
    init_universe(&mut universe);

    for line in contents.split('\n') {
        for col in line.chars() {
            // watch out, x is columns
            // and y is rows
            // but a file is usually indexed by lines, then columns, hence the switch
            universe.cubes.insert(vec![y, x, z], col == '#');

            x += 1;
        }
        x = OFFSET;
        y += 1;
    }

    let mut universe4 = InfiniteCube {
        dimensions: 4,
        cubes: HashMap::new(),
    };

    // Initial state for part 2
    let (mut x, mut y, z, w) = (OFFSET, OFFSET, OFFSET, OFFSET);
    init_universe(&mut universe4);

    for line in contents.split('\n') {
        for col in line.chars() {
            // watch out, x is columns
            // and y is rows
            // but a file is usually indexed by lines, then columns, hence the switch
            universe4.cubes.insert(vec![y, x, z, w], col == '#');

            x += 1;
        }
        x = OFFSET;
        y += 1;
    }

    println!("Cycle 0");
    //print_to_2d(&universe, OFFSET);

    for cycle in 1..=6 {
        //let mut dest = universe.clone();
        let mut dest4 = universe4.clone();
        let mut dest = universe.clone();

        next_cycle(&universe, &mut dest);
        next_cycle(&universe4, &mut dest4);

        universe = dest;
        universe4 = dest4;
        println!("Cycle {}", cycle);
        //print_to_2d(&cube, OFFSET - 1);
        //print_to_2d(&cube, OFFSET);
        //print_to_2d(&cube, OFFSET + 1);
    }

    println!("Active cubes Pt. 1: {}", count_active_cubes(&universe));
    println!("Active cubes Pt. 2: {}", count_active_cubes(&universe4));
    //print_to_2d(&cube, 10);
    // println!("{:?}", game);
    Ok(())
}

fn next_cycle(universe: &InfiniteCube, dest: &mut InfiniteCube) {
    for i in (0..universe.dimensions)
        .map(|_| 0..=OFFSET * 2)
        .multi_cartesian_product()
    {
        let neighbours = count_neighbours(&universe, &i);

        if universe.cubes[&i] && neighbours != 2 && neighbours != 3 {
            // not enough or too many neighbours, become inactive
            dest.cubes.insert(i.clone(), false);
        }
        if !universe.cubes[&i] && neighbours == 3 {
            dest.cubes.insert(i.clone(), true);
        }
    }
}

fn count_neighbours(universe: &InfiniteCube, coords: &[isize]) -> usize {
    let mut neighbours = 0;

    for i in (0..universe.dimensions)
        .map(|_| -1..=1)
        .multi_cartesian_product()
    {
        // don't check yourself
        if i == vec![0; universe.dimensions as usize] {
            continue;
        }

        let new_coords: NDimensions = i
            .into_iter()
            .zip(coords.iter())
            .map(|(a, &b)| a + b)
            .collect();
        //println!("{:?}", new_coords);

        if universe.cubes.contains_key(&new_coords) && universe.cubes[&new_coords] {
            neighbours += 1;
        }
    }

    neighbours
}

fn init_universe(universe: &mut InfiniteCube) {
    for i in (0..universe.dimensions)
        .map(|_| 0..=OFFSET * 2)
        .multi_cartesian_product()
    {
        universe.cubes.insert(i.clone(), false);
    }
}

fn count_active_cubes(universe: &InfiniteCube) -> usize {
    universe.cubes.values().filter(|&elem| *elem).count()
}
