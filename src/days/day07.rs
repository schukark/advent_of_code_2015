use std::collections::HashMap;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::utils;

pub(crate) fn run(part: u32) {
    let input = utils::read_file("inputs/day07.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

#[derive(Parser)]
#[grammar = "config-files/day07.pest"]
pub struct LogicParser;

fn calculate_rule(rule: &Pair<'_, Rule>, state: &HashMap<String, u16>) -> Option<u16> {
    // dbg!(rule.as_str());
    let rule = &rule.clone().into_inner().next().unwrap();

    match rule.as_rule() {
        Rule::operand => {
            let operand = rule.clone().as_str();
            // dbg!(operand);
            if operand.chars().any(|x: char| x.is_ascii_digit()) {
                Some(operand.parse::<u16>().unwrap())
            } else {
                state.get(operand).copied()
            }
        }
        Rule::not_op => {
            let operand = rule.clone().into_inner().as_str();
            // dbg!(format!("NOT {}", operand));
            if operand.chars().any(|x: char| x.is_ascii_digit()) {
                Some(!operand.parse::<u16>().unwrap())
            } else {
                state.get(operand).copied().map(|x| !x)
            }
        }
        Rule::binary_op => {
            let mut inner = rule.clone().into_inner();
            let left_operand = inner.next().unwrap().as_str();
            let operation = inner.next().unwrap().as_str();
            let right_operand = inner.next().unwrap().as_str();

            // dbg!(format!("{} {} {}", left_operand, operation, right_operand));

            let left_operand = if left_operand.chars().any(|x: char| x.is_ascii_digit()) {
                Some(left_operand.parse::<u16>().unwrap())
            } else {
                state.get(left_operand).copied()
            };

            let right_operand = if right_operand.chars().any(|x: char| x.is_ascii_digit()) {
                Some(right_operand.parse::<u16>().unwrap())
            } else {
                state.get(right_operand).copied()
            };

            if let Some(left_value) = left_operand {
                if let Some(right_value) = right_operand {
                    match operation {
                        "AND" => Some(left_value & right_value),
                        "OR" => Some(left_value | right_value),
                        "LSHIFT" => Some(left_value << right_value),
                        "RSHIFT" => Some(left_value >> right_value),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn part1(input: &str) {
    let parse = LogicParser::parse(Rule::file, input)
        .expect("Parse error")
        .next()
        .unwrap();

    let mut state: HashMap<String, u16> = HashMap::new();
    let mut rules: Vec<Pair<'_, Rule>> = Vec::new();

    for rule in parse.into_inner() {
        rules.push(rule);
    }

    let mut changed = true;
    while changed {
        changed = false;

        for rule in &rules {
            if let Rule::EOI = rule.as_rule() {
                break;
            }

            let target = rule.clone().into_inner().nth(1).unwrap().as_str();
            if state.contains_key(target) {
                continue;
            }

            if let Some(result) = calculate_rule(&rule.clone().into_inner().next().unwrap(), &state)
            {
                state.insert(target.to_string(), result);
                changed = true;
                break;
            }
        }
    }

    println!("{:#?}", state.get("a"));
}

fn part2(input: &str) {
    let parse = LogicParser::parse(Rule::file, input)
        .expect("Parse error")
        .next()
        .unwrap();

    let mut state: HashMap<String, u16> = HashMap::new();
    let mut rules: Vec<Pair<'_, Rule>> = Vec::new();

    for rule in parse.into_inner() {
        rules.push(rule);
    }

    let mut changed = true;
    while changed {
        changed = false;

        for rule in &rules {
            if let Rule::EOI = rule.as_rule() {
                break;
            }

            let target = rule.clone().into_inner().nth(1).unwrap().as_str();
            if state.contains_key(target) {
                continue;
            }

            if let Some(result) = calculate_rule(&rule.clone().into_inner().next().unwrap(), &state)
            {
                state.insert(target.to_string(), result);
                changed = true;
                break;
            }
        }
    }

    let a_state = *state.get("a").unwrap();

    let mut state = HashMap::new();
    state.insert("b".to_owned(), a_state);

    changed = true;
    while changed {
        changed = false;

        for rule in &rules {
            if let Rule::EOI = rule.as_rule() {
                break;
            }

            let target = rule.clone().into_inner().nth(1).unwrap().as_str();
            if state.contains_key(target) {
                continue;
            }

            if let Some(result) = calculate_rule(&rule.clone().into_inner().next().unwrap(), &state)
            {
                state.insert(target.to_string(), result);
                changed = true;
                break;
            }
        }
    }

    println!("{:#?}", state.get("a"));
}
