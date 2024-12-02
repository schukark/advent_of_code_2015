use crate::utils;

pub fn run(part: u32) {
    let input = utils::read_file("inputs/day01.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let result = input
        .chars()
        .map(|x| {
            if x == '(' {
                1
            } else if x == ')' {
                -1
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("{}", result);
}

fn part2(input: &str) {
    let result = input
        .chars()
        .map(|x| {
            if x == '(' {
                1
            } else if x == ')' {
                -1
            } else {
                0
            }
        })
        .scan(0, |sum, i| {
            *sum += i;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    println!("{}", result.iter().position(|&x| x < 0).unwrap() + 1);
}
