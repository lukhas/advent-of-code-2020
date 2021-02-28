use std::fs::File;
use std::io::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct NumberHistory {
    first_time: bool,
    oldest: usize,
    youngest: usize,
}

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day15_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // number => (age, number of times it's been spoken)
    let mut numbers: HashMap<usize, NumberHistory> = HashMap::new();
    let last_spoken = game_init(&contents, &mut numbers);
    println!("{:?}", numbers.keys());

    println!(
        "Part 1 (2020): the last number spoken is {}",
        play(2020, last_spoken, &mut numbers)
    );
    // play again
    numbers.clear();
    let last_spoken = game_init(&contents, &mut numbers);
    println!("{:?}", numbers.keys());
    println!(
        "Part 2 (30000000): the last number spoken is {}",
        play(30000000, last_spoken, &mut numbers)
    );

    Ok(())
}

fn say(turn: usize, last_spoken: usize, numbers: &mut HashMap<usize, NumberHistory>) {
    let mut saying = numbers.entry(last_spoken).or_insert(NumberHistory {
        first_time: false,
        oldest: turn,
        youngest: turn,
    });
    saying.first_time = false;
    saying.oldest = saying.youngest;
    saying.youngest = turn;
}

fn play(turns: usize, seed: usize, numbers: &mut HashMap<usize, NumberHistory>) -> usize {
    let mut turn = numbers.len();
    let mut last_spoken = seed;
    while turn < turns {
        turn += 1;
        // if turn % 100000 == 0 {
        //     println!("Turn {}, last spoken was: {}", turn, last_spoken);
        // }

        let &number = numbers.get(&last_spoken).unwrap();
        //dbg!(&number);

        if number.first_time {
            last_spoken = 0;
        } else {
            last_spoken = number.youngest - number.oldest;
        }
        say(turn, last_spoken, numbers);
    }
    last_spoken
}

fn game_init(contents: &str, numbers: &mut HashMap<usize, NumberHistory>) -> usize {
    let mut turn = 1;
    let mut last_spoken = 0;
    for number in contents.split(',') {
        let num: usize = number.trim().parse().unwrap();
        numbers.insert(
            num,
            NumberHistory {
                first_time: true,
                oldest: turn,
                youngest: turn,
            },
        );
        last_spoken = num;
        turn += 1;
    }
    last_spoken
}
