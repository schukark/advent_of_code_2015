use regex::Regex;

use crate::utils;

pub(crate) fn run(part: u32) {
    let input = utils::read_file("inputs/day08.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let re1 = Regex::new(r#"\\(\\|\")"#).unwrap();
    let re2 = Regex::new(r"\\x[0-9a-f]{2}").unwrap();

    let re1_count = re1.find_iter(input).count();
    let re2_count = re2.find_iter(input).count();

    println!(
        "{}",
        (re1_count + 3 * re2_count + 2 * input.lines().count())
    );
}

fn part2(input: &str) {
    todo!()
}
