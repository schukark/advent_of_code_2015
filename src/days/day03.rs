use std::collections::HashSet;

use crate::utils;

pub fn run(part: u32) {
    let input = utils::read_file("inputs/day03.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position = (0, 0);
    visited.insert(current_position);

    input.chars().for_each(|c| {
        current_position = match c {
            '<' => (current_position.0 + 1, current_position.1),
            '>' => (current_position.0 - 1, current_position.1),
            '^' => (current_position.0, current_position.1 + 1),
            'v' => (current_position.0, current_position.1 - 1),
            _ => current_position,
        };
        visited.insert(current_position);
    });

    let present_counts = visited.len();
    println!("{}", present_counts);
}

fn part2(input: &str) {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut current_position1 = (0, 0);
    let mut current_position2 = (0, 0);
    visited.insert(current_position1);
    visited.insert(current_position2);

    let input1 = input
        .chars()
        .enumerate()
        .filter(|(idx, _val)| idx % 2 == 0)
        .map(|(_idx, val)| val)
        .collect::<Vec<_>>();

    let input2 = input
        .chars()
        .enumerate()
        .filter(|(idx, _val)| idx % 2 == 1)
        .map(|(_idx, val)| val)
        .collect::<Vec<_>>();

    input1.iter().for_each(|c| {
        current_position1 = match c {
            '<' => (current_position1.0 + 1, current_position1.1),
            '>' => (current_position1.0 - 1, current_position1.1),
            '^' => (current_position1.0, current_position1.1 + 1),
            'v' => (current_position1.0, current_position1.1 - 1),
            _ => current_position1,
        };
        visited.insert(current_position1);
    });

    input2.iter().for_each(|c| {
        current_position2 = match c {
            '<' => (current_position2.0 + 1, current_position2.1),
            '>' => (current_position2.0 - 1, current_position2.1),
            '^' => (current_position2.0, current_position2.1 + 1),
            'v' => (current_position2.0, current_position2.1 - 1),
            _ => current_position2,
        };
        visited.insert(current_position2);
    });

    let present_counts = visited.len();
    println!("{}", present_counts);
}
