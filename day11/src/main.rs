
use std::fs::File;
use std::io::prelude::*;

use ndarray::{Array2, Axis};

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day11_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // ugly, but meh
    let height = contents.matches('\n').count();
    let width  = contents.split('\n').next().unwrap().chars().count();

    let mut initial_board = Array2::<char>::default((height,width));

    let mut col = 0;
    let mut row = 0;
    for c in contents.chars() {
        //println!("xy: {} {} - c: {}", x, y, c);
        if c == '\n' {
            col = 0;
            row += 1;
            continue;
        }
        initial_board[[row,col]] = c;
        col += 1;
    }

    //println!("Initial layout:");
    //print_board(&initial_board);

    let mut count = 0;
    loop {
        let mut next = initial_board.clone();
        count += 1;

        //println!("\nIteration {}", count);
        let changed = next_round(&initial_board, &mut next);
        //print_board(&next);

        if !changed {
            break
        }
        initial_board = next.clone();
    }

    println!("\nTotal iterations: {}", count);
    println!("Occupied seats: {}", initial_board.iter().filter(|&elem| elem == &'#').count());
    Ok(())
}

fn next_round(src: &Array2::<char>, dest: &mut Array2::<char>) -> bool {
    let mut col = 0;
    let mut row = 0;
    let mut changed = false;
    
    while row < dest.shape()[0] {
        while col < dest.shape()[1] {
            // look around for neighbours
            //println!("Looking at {} [{},{}]", src[[row,col]], row, col);
            let neighbours = get_neighbours(&src, &[row,col]);
            //println!("Neighbours: {:?}",neighbours);
            
            // empty seat
            if src[[row,col]] == 'L' && neighbours.iter().filter(|&elem| elem == &'#').count() == 0 {
                dest[[row,col]] = '#';
                changed = true;
            }

            // occupied seat
            if src[[row,col]] == '#' && neighbours.iter().filter(|&elem| elem == &'#').count() >= 4 {
                dest[[row,col]] = 'L';
                changed = true;
            }
            col+= 1;

        }
        row+= 1;
        col = 0;
    }
    changed
}

fn get_neighbours(board: &Array2::<char>, coords: &[usize]) -> Vec<char> {
    let mut neighbours: Vec<char> = Vec::new();

    // neighbours one row up?
    if coords[0] > 0 {
        // neighbours one col left?
        if coords[1] > 0 {
            neighbours.push( board[[ coords[0] - 1, coords[1] - 1 ]] );
        }
        neighbours.push( board[[ coords[0] - 1, coords[1] ]] );
        // neighbours one col right?
        if coords[1] < (board.shape()[1] - 1)  {
            neighbours.push( board[[ coords[0] - 1, coords[1] + 1 ]] );
        }
    }

    // neighbours left and right?
    if coords[1] > 0 {
        neighbours.push( board[[ coords[0], coords[1] - 1 ]] );
    }
    if coords[1] < (board.shape()[1] - 1)  {
        neighbours.push( board[[ coords[0], coords[1] + 1 ]] );
    }
    
    // neighbours one row up?
    if coords[0] < (board.shape()[0] - 1) {
        // neighbours one col left?
        if coords[1] > 0 {
            neighbours.push( board[[ coords[0] + 1, coords[1] - 1 ]] );
        }
        neighbours.push( board[[ coords[0] + 1, coords[1] ]] );
        // neighbours one col right?
        if coords[1] < (board.shape()[1] - 1)  {
            neighbours.push( board[[ coords[0] + 1, coords[1] + 1 ]] );
        }
    }

    neighbours
}

fn print_board(board: &Array2::<char>) {
    for row in board.axis_iter(Axis(0)) {
        for col in row.axis_iter(Axis(0)) {
            print!("{}",col);
        }
        print!("\n");
    }
}
