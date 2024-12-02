use std::collections::HashMap;

use crate::utils;

pub fn run(part: u32) {
    let input = utils::read_file("inputs/day05.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn is_valid(input: &str) -> bool {
    let vowels = "aeiou";
    let banned = [
        "ab".to_owned(),
        "cd".to_owned(),
        "pq".to_owned(),
        "xy".to_owned(),
    ];

    input.chars().filter(|&x| vowels.contains(x)).count() >= 3
        && input
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|x| x[0] == x[1])
        && !input
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .any(|x| banned.contains(&x.iter().collect::<String>()))
}

fn part1(input: &str) {
    let count = input.lines().filter(|&x| is_valid(x)).count();
    println!("{}", count);
}

fn is_valid_extended(input: &str) -> bool {
    let mut pair_index: HashMap<String, Vec<usize>> = HashMap::new();
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .for_each(|(index, x)| {
            let pair = x.iter().collect::<String>();
            pair_index.entry(pair).or_default().push(index);
        });

    let two_pairs = pair_index
        .iter()
        .any(|(_key, value)| value.first().unwrap() + 1 < *value.last().unwrap());

    let sandwich = input
        .chars()
        .collect::<Vec<char>>()
        .windows(3)
        .any(|x| x[0] == x[2]);

    two_pairs && sandwich
}

fn part2(input: &str) {
    let count = input.lines().filter(|&x| is_valid_extended(x)).count();
    println!("{}", count);
}
