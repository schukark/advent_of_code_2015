use crate::utils;

pub fn run(part: u32) {
    let input = utils::read_file("inputs/day02.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn calculate_paper(sides: &[i32]) -> i32 {
    let mut sorted_sides = [0, 0, 0];
    sorted_sides.clone_from_slice(sides);
    sorted_sides.sort_unstable();

    2 * (sides[0] * sides[1] + sides[0] * sides[2] + sides[1] * sides[2])
        + sorted_sides[0] * sorted_sides[1]
}

fn calculate_ribbon(sides: &[i32]) -> i32 {
    let mut sorted_sides = [0, 0, 0];
    sorted_sides.clone_from_slice(sides);
    sorted_sides.sort_unstable();

    2 * (sorted_sides[0] + sorted_sides[1]) + sorted_sides.iter().product::<i32>()
}

fn part1(input: &str) {
    let result = input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|sides| calculate_paper(&sides))
        .sum::<i32>();

    println!("{result}");
}

fn part2(input: &str) {
    let result = input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|sides| calculate_ribbon(&sides))
        .sum::<i32>();

    println!("{result}");
}
