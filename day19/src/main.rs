use std::fs::File;
use std::io::prelude::*;

use std::process::exit;

use std::collections::{HashMap, HashSet};

type Rules = Vec<usize>;

#[derive(Debug)]
struct Rule {
    id: usize,
    rules: (Rules, Rules),
    matches: String,
}

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day19_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    let mut rules: HashMap<usize, Rule> = HashMap::new();

    let rules_header = contents.split("\n\n").next().unwrap();

    parse_rules(&rules_header, &mut rules);

    let mut possible_messages: Vec<String> = vec!["".to_string()];

    generate_messages(&mut possible_messages, &rules[&0], &rules);

    //println!("{:?}", possible_messages.len());

    let mut mh: HashSet<&str> = HashSet::new();
    for line in contents.split("\n\n").nth(1).unwrap().split('\n') {
        //println!("Line: {}", &line);
        mh.insert(line);
    }

    let number_matching = possible_messages
        .iter()
        .filter(|m| mh.contains(m.as_str()))
        .count();

    println!("Number matching: {}", number_matching);

    Ok(())
}

fn parse_rules(text: &str, rules: &mut HashMap<usize, Rule>) {
    for mut r in text.split('\n') {
        let idx;
        let left;
        let right;
        if let Some(x) = r.find(':') {
            idx = r[..x].parse().unwrap();
            r = &r[x + 1..];
        } else {
            // no rule, don't bother
            continue;
        }
        // complex cases first
        if let Some(x) = r.find('|') {
            // skip leading and trailing space
            left = r[1..x - 1]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            right = r[x + 2..r.len()]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            rules.insert(
                idx,
                Rule {
                    id: idx,
                    matches: "".to_string(),
                    rules: (left, right),
                },
            );
        } else if let Some(x) = r.find('"') {
            let m = r[x..].to_string().replace("\"", "");
            rules.insert(
                idx,
                Rule {
                    id: idx,
                    matches: m,
                    rules: (Vec::new(), Vec::new()),
                },
            );
        } else {
            left = r[1..]
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            rules.insert(
                idx,
                Rule {
                    id: idx,
                    matches: "".to_string(),
                    rules: (left, Vec::new()),
                },
            );
        }
    }
}

fn generate_messages(
    messages: &mut Vec<String>,
    current_rule: &Rule,
    rules: &HashMap<usize, Rule>,
) {
    // println!(
    //     "Current state : {:?} | Generating from {:?}",
    //     messages, current_rule
    // );

    if current_rule.matches.is_empty() {
        if current_rule.rules.1.is_empty() {
            for r in current_rule.rules.0.iter() {
                generate_messages(messages, &rules[&r], rules);
            }
        } else {
            // need to apply left and right in "parallel"
            let mut forked_messages = messages.clone();

            for r in current_rule.rules.0.iter() {
                generate_messages(messages, &rules[&r], rules);
            }

            for r in current_rule.rules.1.iter() {
                generate_messages(&mut forked_messages, &rules[&r], rules);
            }

            messages.append(&mut forked_messages);
        }
    } else {
        for m in messages.iter_mut() {
            m.push_str(&current_rule.matches);
        }
    }
}
