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

#[derive(Debug)]
struct Waypoint {
    offset: Coordinates,
    boat_pos: Position,
}

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day12_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut boat_pos = Position {
        coords: Coordinates { east: 0, north: 0 },
        direction: 90,
    };

    let mut waypoint_pos = Waypoint {
        offset: Coordinates { east: 10, north: 1 },
        boat_pos: Position {
            coords: Coordinates { east: 0, north: 0 },
            direction: 90,
        },
    };

    // println!("Initial Position");
    // println!("{:?}", pos);

    for line in contents.split('\n') {
        if line.len() > 1 {
            let direction = line.chars().next().unwrap();
            let magnitude: i32 = line.get(1..).unwrap().parse().unwrap();
            //println!("{} - {}", direction, magnitude);
            move_boat(&mut boat_pos, direction, magnitude);
            //println!("{:?}", pos);
            move_waypoint(&mut waypoint_pos, direction, magnitude);
            //println!("Offset: {:?}", waypoint_pos.offset);
            //println!("Boat: {:?}\n", waypoint_pos.boat_pos);
        }
    }

    println!("Manhattan distance pt.1: {}", manhattan(&boat_pos.coords));
    println!(
        "Manhattan distance pt.2: {}",
        manhattan(&waypoint_pos.boat_pos.coords)
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

fn move_waypoint(waypoint: &mut Waypoint, direction: char, magnitude: i32) {
    match direction {
        'N' => {
            waypoint.offset.north += magnitude;
        }
        'S' => {
            waypoint.offset.north -= magnitude;
        }
        'E' => {
            waypoint.offset.east += magnitude;
        }
        'W' => {
            waypoint.offset.east -= magnitude;
        }
        'R' => {
            // clockwise rotation
            //println!("clockwise rotation of {}", magnitude);
            //println!("  current: {:?}", waypoint.offset);
            waypoint.offset = rotate(&waypoint.offset, magnitude);
            //println!("  after: {:?}", waypoint.offset);
        }
        'L' => {
            // anti-clockwise rotation
            //println!("anticlockwise rotation of {}", magnitude);
            //println!("  current: {:?}", waypoint.offset);
            // keep it positive for simplicity, it's a circle
            waypoint.offset = rotate(&waypoint.offset, (-magnitude + 360) % 360);
            //println!("  after: {:?}", waypoint.offset);
        }
        'F' => {
            // apply the offset magnitude times
            //println!("Moving forward by {}", magnitude);
            for _ in 1..=magnitude {
                waypoint.boat_pos.coords.north += waypoint.offset.north;
                waypoint.boat_pos.coords.east += waypoint.offset.east;
            }
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

fn rotate(coords: &Coordinates, angle: i32) -> Coordinates {
    let (cos, sin) = match angle {
        90 => (0, 1),
        180 => (-1, 0),
        270 => (0, -1),
        360 => (1, 0),
        _ => (0, 0),
    };
    // see https://en.wikipedia.org/wiki/Rotation_of_axes
    Coordinates {
        east: coords.east * cos + coords.north * sin,
        north: -coords.east * sin + coords.north * cos,
    }
}

fn manhattan(coords: &Coordinates) -> i32 {
    coords.north.abs() + coords.east.abs()
}
