use std::fs::File;
use std::io::prelude::*;
use std::collections::{HashMap};

#[macro_use] extern crate lazy_static;
use regex::Regex;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day7_input")?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    lazy_static! {
        static ref RE_HEAD: Regex    = Regex::new(r"(?m)^(.*) bags contain (.*).").unwrap();
        static ref RE_CONTENT: Regex = Regex::new(r"([0-9]+) (.*?) bag").unwrap();
    }

    let mut bags: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in RE_HEAD.captures_iter(&contents) {
        let parent       = line.get(1).unwrap().as_str();
        let bag_contents = line.get(2).unwrap().as_str();
        let mut child: HashMap<String, usize> = HashMap::new();
        
        for bag in RE_CONTENT.captures_iter(bag_contents) {
            //println!("Bag: {:?}", bag);
            let qty: usize = bag.get(1).unwrap().as_str().parse().unwrap();
            let bag = bag.get(2).unwrap().as_str();

            child.insert(bag.to_owned(), qty);
        }
        bags.entry(parent.to_owned()).or_insert(child);
    }

    println!("Count pt. 1: {}", bags.iter().filter(|(k, _) | find_gold(&bags,k)).count());
    println!("Count pt. 2: {}", count_contenants(&bags, "shiny gold") - 1);
    
    Ok(())
}

fn count_contenants(bags: &HashMap<String, HashMap<String, usize>>, key: &str) -> usize {
    //println!("counting {}",key);
    match &bags.get(key) {
        None => 0,
        Some(h) =>
        {
            //println!("  {:?}",h);
            h.iter().fold(1, | found, (elem, qty) | {
                found + qty * count_contenants(&bags, elem)
            })
        }
    }
}

fn find_gold(bags: &HashMap<String, HashMap<String, usize>>, key: &str) -> bool {
    match &bags.get(key) {
        None => false,
        Some(h) =>
        {
            h.iter().fold(false, | found, (elem, _) | {
                if elem == "shiny gold" {
                    true
                } else {
                    found || find_gold(&bags, elem)
                }
            })
        }
    }
}

