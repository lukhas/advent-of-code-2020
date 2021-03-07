use std::fs::File;
use std::io::prelude::*;

use lazy_static::lazy_static;
use regex::Regex;

use std::collections::BTreeMap;

// always (lower, upper)
type Boundary = (usize, usize);

type Ticket = Vec<usize>;

#[derive(Debug, Clone)]
struct Field {
    field_name: String,
    boundaries: Vec<(usize, usize)>,
}

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day16_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    lazy_static! {
        static ref RE_FIELD: Regex = Regex::new(
            r"(?m)^(?P<name>.*):\s+(?P<lower1>\d+)-(?P<upper1>\d+) or (?P<lower2>\d+)-(?P<upper2>\d+)"
        )
        .unwrap();
        static ref RE_TICKET: Regex = Regex::new(r"(?m)^\d+,.*\d$").unwrap();
    }

    let mut fields: Vec<Field> = Vec::new();

    // first part: allowed ranges in fields
    for cap in RE_FIELD.captures_iter(&contents) {
        // parse().unwrap() is fine here since we regex'ed on \d above
        let b1: Boundary = (
            cap["lower1"].parse().unwrap(),
            cap["upper1"].parse().unwrap(),
        );
        let b2: Boundary = (
            cap["lower2"].parse().unwrap(),
            cap["upper2"].parse().unwrap(),
        );
        let v = vec![b1, b2];
        let field = Field {
            field_name: cap["name"].to_string(),
            boundaries: v,
        };

        fields.push(field);
    }

    let mut my_ticket: Ticket = vec![];

    let mut valid_tickets: Vec<Ticket> = vec![];

    let mut pt1_error_rate: usize = 0;

    for cap in RE_TICKET.captures_iter(&contents) {
        //println!("{:?}", cap);

        // the first ticket is our ticket
        if my_ticket.is_empty() {
            my_ticket = line_to_ticket(&cap[0]);
            //println!("My ticket: {:?}", my_ticket);
        } else {
            // looking at other tickets
            // Pt1 wants to find the ones that do not comply with requirements
            let ticket = line_to_ticket(&cap[0]);

            let mut incremented = false;
            //println!("\nLooking at: {:?}", ticket);

            for val in ticket.iter() {
                //println!("{:?}", val);
                if fields
                    .iter()
                    .filter(|f| value_in_ticket_boundary(val, f))
                    .count()
                    == 0
                {
                    //println!("  invalid for field");
                    pt1_error_rate += val;
                    incremented = true;
                }
            }
            if !incremented {
                valid_tickets.push(ticket.clone());
            }
        }
    }

    println!("Pt1 error rate: {}", pt1_error_rate);

    //println!("Valid tickets: {:?}", valid_tickets);

    let mut matched = BTreeMap::new();

    for f in fields.iter() {
        // println!("Looking at {}", f.field_name);
        // println!("  Boundaries: {:?}", f.boundaries);
        for idx in 0..my_ticket.len() {
            let vertical_values = valid_tickets.iter().map(|t| t[idx]).collect::<Vec<usize>>();
            // println!("    {:?}", vertical_values);

            if values_fit_boundaries(&vertical_values, &f.boundaries) {
                let h = matched.entry(&f.field_name).or_insert_with(BTreeMap::new);
                h.insert(idx, true);
            }
        }
    }

    //println!("{:?}", matched);

    let mut final_pass: BTreeMap<&String, BTreeMap<usize, bool>> = BTreeMap::new();

    let matched_len = matched.len();
    while final_pass.len() < matched_len {
        let mut copy = matched.clone();
        for (k, v) in matched.iter_mut() {
            if v.is_empty() {
                copy.remove(k);
            }
            // only one element, that's our next single solution
            if v.len() == 1 {
                // ugh
                let lone_field = v.keys().next().unwrap();

                final_pass.insert(k, v.clone());
                // remove this elem from the whole btree
                for (_kk, vv) in copy.iter_mut() {
                    vv.remove(lone_field);
                }
            }
        }
        matched = copy;
    }
    // println!();
    // println!("{:?}", final_pass);
    // println!("{:?}", matched);

    // Find out what's on my ticket
    let mut pt2_result = 1;
    for (k, v) in final_pass {
        let field = v.keys().next().unwrap();
        // println!("{} is {}", k, my_ticket[*field]);

        if k.starts_with("departure") {
            pt2_result *= my_ticket[*field];
        }
    }

    println!("Pt2 result: {}", pt2_result);
    Ok(())
}

fn line_to_ticket(line: &str) -> Ticket {
    line.split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>()
}

fn value_in_ticket_boundary(value: &usize, field: &Field) -> bool {
    field
        .boundaries
        .iter()
        .filter(|bound| value_in_boundary(value, bound))
        .count()
        > 0
}

fn value_in_boundary(value: &usize, boundary: &Boundary) -> bool {
    *value >= boundary.0 && *value <= boundary.1
}

fn values_fit_boundaries(values: &[usize], boundaries: &[Boundary]) -> bool {
    values
        .iter()
        .all(|v| boundaries.iter().any(|b| value_in_boundary(v, &b)))
}
