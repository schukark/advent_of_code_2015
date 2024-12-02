use crate::utils;

pub fn run(part: u32) {
    let input = utils::read_file("inputs/day04.txt");

    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => println!("Invalid part"),
    }
}

fn part1(input: &str) {
    let result = (0..)
        .map(|x| (x, md5::compute(input.to_owned() + &x.to_string())))
        .find(|(_x, val)| format!("{:x}", val)[..5].chars().all(|x| x == '0'));

    if let Some((x, _val)) = result {
        println!("{}", x);
    } else {
        println!("No match found");
    }
}

#[test]
fn test_md5() {
    let digest = md5::compute("abcdef609043");
    assert!(format!("{:x}", &digest)[..5].chars().all(|x| x == '0'));
}

fn part2(input: &str) {
    let result = (0..)
        .map(|x| (x, md5::compute(input.to_owned() + &x.to_string())))
        .find(|(_x, val)| format!("{:x}", val)[..6].chars().all(|x| x == '0'));

    if let Some((x, _val)) = result {
        println!("{}", x);
    } else {
        println!("No match found");
    }
}
