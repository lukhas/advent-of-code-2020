use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use std::convert::TryInto;

use regex::Regex;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day6_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    let mut total_anyone   = 0;
    let mut total_everyone = 0;
    
    for group in contents.split("\n\n") {
        let g = build_group(group);
        total_anyone += g.answers.len();

        for (_, answer_count) in g.answers {
            if answer_count == g.persons {
                total_everyone += 1;
            }
        }
    }

    println!("Total count anyone   (pt.1): {}",total_anyone);
    println!("Total count everyone (pt.2): {}",total_everyone);
    Ok(())
}

fn count_persons(group: &str) -> u32 {
    (group.matches("\n").count() + 1).try_into().unwrap()
}

struct Group<'a> {
    persons: u32,
    answers: HashMap<&'a str,u32>,
}

fn build_group(group: &str) -> Group {
    let mut answers: HashMap<&str,u32> = HashMap::new();
    let re = Regex::new(r"[a-z]").unwrap();
    re.find_iter(group).for_each(|answer| {
        let count = answers.entry(answer.as_str()).or_insert(0);
        *count += 1;
    });
    
    Group {
        persons: count_persons(group),
        answers: answers,
    }
}
