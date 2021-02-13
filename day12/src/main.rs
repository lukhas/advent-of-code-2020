use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Coordinates {
    east: i32,
    north: i32,
}

#[derive(Debug)]
struct Position {
    coords: Coordinates,
    direction: i32,
}

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day12_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut pos = Position {
        coords: Coordinates { east: 0, north: 0 },
        direction: 90,
    };

    println!("Initial Position");
    println!("{:?}", pos);

    for line in contents.split('\n') {
        if line.len() > 1 {
            let direction = line.chars().next().unwrap();
            let magnitude: i32 = line.get(1..).unwrap().parse().unwrap();
            //println!("{} - {}", direction, magnitude);
            move_boat(&mut pos, direction, magnitude);
            println!("{:?}", pos);
        }
    }

    println!(
        "Manhattan distance: {}",
        pos.coords.north.abs() + pos.coords.east.abs()
    );
    Ok(())
}

fn move_boat(pos: &mut Position, direction: char, magnitude: i32) {
    match direction {
        'N' => {
            pos.coords.north += magnitude;
        }
        'S' => {
            pos.coords.north -= magnitude;
        }
        'E' => {
            pos.coords.east += magnitude;
        }
        'W' => {
            pos.coords.east -= magnitude;
        }
        'F' => {
            move_boat(pos, direction_to_compass(pos.direction), magnitude);
        }
        'R' => {
            pos.direction = (pos.direction + magnitude) % 360;
        }
        'L' => {
            // keep it positive for simplicity, it's a circle
            pos.direction = (pos.direction - magnitude + 360) % 360;
        }

        _ => println!("IDK"),
    }
}

fn direction_to_compass(direction: i32) -> char {
    match direction {
        0 => 'N',
        90 => 'E',
        180 => 'S',
        270 => 'W',
        _ => 'X',
    }
}
