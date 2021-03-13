use std::fs::File;
use std::io::prelude::*;

// use lazy_static::lazy_static;
// use regex::Regex;

fn main() -> std::io::Result<()> {
    // file must be in the directory from where we call the executable
    let mut file = File::open("day18_input")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    contents = contents.trim().to_string();

    let mut pt1_result = 0;
    let mut pt2_result = 0;

    for line in contents.split('\n') {
        //println!("{}", line);

        let res = parse_expr(
            &line
                .replace("(", "( ")
                .replace(")", " )")
                .split(' ')
                .collect::<Vec<&str>>(),
            false,
        );

        // weird precedence rules
        let res2 = parse_expr(
            &line
                .replace("(", "( ")
                .replace(")", " )")
                .split(' ')
                .collect::<Vec<&str>>(),
            true,
        );

        //println!("  {}", res);
        //println!("  {}", res2);
        pt1_result += res;
        pt2_result += res2;
    }

    println!("Pt. 1: {}", pt1_result);
    println!("Pt. 2: {}", pt2_result);
    Ok(())
}

fn parse_expr(expr: &[&str], weird_precedence: bool) -> usize {
    //println!("  Parsing {:?}", expr);

    // innermost paren will be the rightmost opening paren
    match expr.iter().rposition(|&x| x == "(") {
        Some(inner_open) => {
            // find the matching paren to the innermost
            let inner_close = inner_open
                + expr
                    .iter()
                    .skip(inner_open)
                    .position(|&x| x == ")")
                    .unwrap();

            let partial_result =
                parse_expr(&expr[inner_open + 1..=inner_close - 1], weird_precedence).to_string();

            let recur_expr = [
                &expr[..inner_open],
                &[partial_result.as_str()],
                &expr[inner_close + 1..],
            ]
            .concat();

            parse_expr(&recur_expr, weird_precedence)
        }
        // no parens, easy parsing job
        None => {
            if weird_precedence {
                // TODO pt2 I'm sure
                if expr.len() == 1 {
                    expr[0].parse::<usize>().unwrap()
                } else {
                    // additions take precedence over multiplications
                    //println!("  Looking at {:?}", expr);

                    match expr.iter().position(|&x| x == "+") {
                        Some(operator_position) => {
                            // found an addition, evaluate first
                            //println!("    Evaluating addition first");

                            let left_index = operator_position - 1;
                            let right_index = operator_position + 1;
                            let partial_result =
                                compute(&expr[left_index..=right_index].to_vec()).to_string();
                            // pass the remainder to the recursion
                            let recur_expr = [
                                &expr[..left_index],
                                &[partial_result.as_str()],
                                &expr[right_index + 1..],
                            ]
                            .concat();

                            //println!("  reminder: {:?}", recur_expr);
                            parse_expr(&recur_expr, weird_precedence)
                        }
                        None => {
                            let partial_result = compute(&expr[0..=2].to_vec()).to_string();
                            // pass the remainder to the recursion
                            let mut recur_expr = expr[3..].to_vec();
                            recur_expr.insert(0, partial_result.as_str());

                            //println!("  reminder: {:?}", recur_expr);
                            parse_expr(&recur_expr, weird_precedence)
                        }
                    }
                }
            } else {
                // single element, just return it
                if expr.len() == 1 {
                    //println!("returning simple elem");
                    expr[0].parse::<usize>().unwrap()
                } else {
                    let partial_result = compute(&expr[0..=2].to_vec()).to_string();
                    // pass the remainder to the recursion
                    let mut recur_expr = expr[3..].to_vec();
                    recur_expr.insert(0, partial_result.as_str());

                    //println!("  reminder: {:?}", recur_expr);
                    parse_expr(&recur_expr, weird_precedence)
                }
            }
        }
    }
}

fn compute(expr: &[&str]) -> usize {
    let mut left: usize = expr[0].parse().unwrap();
    if expr[1] == "*" {
        left *= expr[2].parse::<usize>().unwrap();
    } else if expr[1] == "+" {
        left += expr[2].parse::<usize>().unwrap();
    }
    //println!("    Computing {:?}  => {}", expr, left);
    left
}
