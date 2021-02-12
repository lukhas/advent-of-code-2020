use std::fs::File;
use std::io::prelude::*;

use ndarray::{Array2, Axis};

type Board = Array2<char>;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day11_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // ugly, but meh
    let height = contents.matches('\n').count();
    let width = contents.split('\n').next().unwrap().chars().count();

    let mut initial_board = Board::default((height, width));

    let mut col = 0;
    let mut row = 0;
    for c in contents.chars() {
        //println!("xy: {} {} - c: {}", x, y, c);
        if c == '\n' {
            col = 0;
            row += 1;
            continue;
        }
        initial_board[[row, col]] = c;
        col += 1;
    }

    /////////
    // Part 1

    let mut starting_board = initial_board.clone();
    //println!("Initial layout:");
    //print_board(&starting_board);

    loop {
        let mut next = starting_board.clone();

        let changed = next_round(&starting_board, &mut next, 4, &get_immediate_neighbours);
        //print_board(&next);

        if !changed {
            break;
        }
        starting_board = next.clone();
    }

    println!(
        "Occupied seats pt. 1: {}",
        starting_board.iter().filter(|&elem| elem == &'#').count()
    );

    /////////
    // Part 2

    starting_board = initial_board;
    //println!("Initial layout:");
    //print_board(&starting_board);

    loop {
        let mut next = starting_board.clone();

        let changed = next_round(&starting_board, &mut next, 5, &get_seat_neighbours);
        //print_board(&next);

        if !changed {
            break;
        }
        starting_board = next.clone();
    }

    println!(
        "Occupied seats pt. 2: {}",
        starting_board.iter().filter(|&elem| elem == &'#').count()
    );
    Ok(())
}

fn next_round(
    src: &Board,
    dest: &mut Board,
    occupancy_limit: usize,
    neighbour_fn: &dyn Fn(&Board, &[usize]) -> Vec<char>,
) -> bool {
    let mut col = 0;
    let mut row = 0;
    let mut changed = false;

    while row < dest.shape()[0] {
        while col < dest.shape()[1] {
            if src[[row, col]] == '.' {
                col += 1;
                continue;
            }
            // look around for neighbours
            //println!("Looking at {} [{},{}]", src[[row,col]], row, col);
            let neighbours = neighbour_fn(&src, &[row, col]);
            //println!("Neighbours: {:?}",neighbours);

            // empty seat
            if src[[row, col]] == 'L' && neighbours.iter().filter(|&elem| elem == &'#').count() == 0
            {
                dest[[row, col]] = '#';
                changed = true;
            }

            // occupied seat
            if src[[row, col]] == '#'
                && neighbours.iter().filter(|&elem| elem == &'#').count() >= occupancy_limit
            {
                dest[[row, col]] = 'L';
                changed = true;
            }
            col += 1;
        }
        row += 1;
        col = 0;
    }
    changed
}

fn get_immediate_neighbours(board: &Board, coords: &[usize]) -> Vec<char> {
    let mut neighbours: Vec<char> = Vec::new();
    let row = coords[0];
    let col = coords[1];

    // neighbours one row up?
    if row > 0 {
        // neighbours one col left?
        if col > 0 {
            neighbours.push(board[[row - 1, col - 1]]);
        }
        neighbours.push(board[[row - 1, col]]);
        // neighbours one col right?
        if col < (board.shape()[1] - 1) {
            neighbours.push(board[[row - 1, col + 1]]);
        }
    }

    // neighbours left and right?
    if col > 0 {
        neighbours.push(board[[row, col - 1]]);
    }
    if col < (board.shape()[1] - 1) {
        neighbours.push(board[[row, col + 1]]);
    }

    // neighbours one row down?
    if row < (board.shape()[0] - 1) {
        // neighbours one col left?
        if col > 0 {
            neighbours.push(board[[row + 1, col - 1]]);
        }
        neighbours.push(board[[row + 1, col]]);
        // neighbours one col right?
        if col < (board.shape()[1] - 1) {
            neighbours.push(board[[row + 1, col + 1]]);
        }
    }

    neighbours
}

fn get_seat_neighbours(board: &Board, coords: &[usize]) -> Vec<char> {
    let mut neighbours: Vec<char> = Vec::new();

    // top left diag
    if let Some(x) = get_nearest_seat(board, coords, -1, -1) {
        neighbours.push(x);
    }
    // top center
    if let Some(x) = get_nearest_seat(board, coords, -1, 0) {
        neighbours.push(x);
    }
    // top right diag
    if let Some(x) = get_nearest_seat(board, coords, -1, 1) {
        neighbours.push(x);
    }
    // left
    if let Some(x) = get_nearest_seat(board, coords, 0, -1) {
        neighbours.push(x);
    }
    // right
    if let Some(x) = get_nearest_seat(board, coords, 0, 1) {
        neighbours.push(x);
    }
    // bottom left diag
    if let Some(x) = get_nearest_seat(board, coords, 1, -1) {
        neighbours.push(x);
    }
    // bottom center
    if let Some(x) = get_nearest_seat(board, coords, 1, 0) {
        neighbours.push(x);
    }
    // bottom right diag
    if let Some(x) = get_nearest_seat(board, coords, 1, 1) {
        neighbours.push(x);
    }

    neighbours
}

fn get_nearest_seat(
    board: &Board,
    coords: &[usize],
    v_direction: isize,
    h_direction: isize,
) -> Option<char> {
    let mut row = coords[0] as isize;
    let mut col = coords[1] as isize;

    loop {
        // get special cases out of the way
        if (v_direction < 0 && (row == 0))
            || (v_direction > 0 && ((row as usize) == board.shape()[0] - 1))
            || (h_direction < 0 && (col == 0))
            || (h_direction > 0 && ((col as usize) == board.shape()[1] - 1))
        {
            //println!("Skipping {} {}",v_direction,h_direction);
            return None;
        }

        row += v_direction;
        col += h_direction;

        if board[[(row as usize), (col as usize)]] != '.' {
            return Some(board[[(row as usize), (col as usize)]]);
        }
    }
}

#[allow(dead_code)]
fn print_board(board: &Board) {
    for row in board.axis_iter(Axis(0)) {
        for col in row.axis_iter(Axis(0)) {
            print!("{}", col);
        }
        println!();
    }
}
