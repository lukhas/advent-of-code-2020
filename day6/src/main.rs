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

    let mut total_count   = 0;
    let mut total_count_2 = 0;
    let re = Regex::new(r"[a-z]").unwrap();
    
    for group in contents.split("\n\n") {
        let people_count:u32 = (group.matches("\n").count() + 1).try_into().unwrap();
        //println!("People_Count: {}",people_count);

        let mut answers: HashMap<&str,u32> = HashMap::new();
        re.find_iter(group).for_each(|answer| {
            let count = answers.entry(answer.as_str()).or_insert(0);
            *count += 1;
        });

        for (_, v) in &answers {
            if *v == people_count {
                total_count_2 += 1;
            }
        }

        //println!("Map: {:?}\n",answers);
        total_count += answers.len();
        //println!("Group count: {}\n",answers.len());
    }

    println!("Total count pt.1: {}",total_count);
    println!("Total count pt.2: {}",total_count_2);
    Ok(())
}
