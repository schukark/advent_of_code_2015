use crate::utils;

const SIZE: usize = 1000;

struct Board {
    lights: [[bool; SIZE]; SIZE],
}

impl Board {
    fn new() -> Board {
        Board {
            lights: [[false; SIZE]; SIZE],
        }
    }

    fn turn_off(&mut self, left_corner: (i32, i32), right_corner: (i32, i32)) {
        for x in left_corner.0..=right_corner.0 {
            for y in left_corner.1..=right_corner.1 {
                self.lights[x as usize][y as usize] = false;
            }
        }
    }

    fn turn_on(&mut self, left_corner: (i32, i32), right_corner: (i32, i32)) {
        for x in left_corner.0..=right_corner.0 {
            for y in left_corner.1..=right_corner.1 {
                self.lights[x as usize][y as usize] = true;
            }
        }
    }

    fn toggle(&mut self, left_corner: (i32, i32), right_corner: (i32, i32)) {
        for x in left_corner.0..=right_corner.0 {
            for y in left_corner.1..=right_corner.1 {
                self.lights[x as usize][y as usize] ^= true;
            }
        }
    }

    fn count_lit_lights(&self) -> usize {
        self.lights.iter().flatten().filter(|&x| *x).count()
    }
}

pub fn run(part: u32) {
    let input = utils::read_file("inputs/day06.txt");

    match part {
        1 => part1(input),
        2 => part2(input),
        _ => println!("Invalid part"),
    }
}

fn parse_instructions(input: &str) -> Vec<(String, (i32, i32), (i32, i32))> {
    input.lines().map(parse_one_line).collect()
}

fn parse_one_line(input: &str) -> (String, (i32, i32), (i32, i32)) {
    let collect = input.split_whitespace().collect::<Vec<&str>>();
    let right_corner = collect.last().unwrap();
    let left_corner = collect[collect.len() - 3];

    let left_corner = left_corner
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let right_corner = right_corner
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    let command = collect[..collect.len() - 3].join(" ");

    (
        command,
        (left_corner[0], left_corner[1]),
        (right_corner[0], right_corner[1]),
    )
}

fn part1(input: String) {
    let mut board = Board::new();
    let instructions = parse_instructions(&input);

    for (command, left_corner, right_corner) in instructions {
        match command.as_str() {
            "turn on" => board.turn_on(left_corner, right_corner),
            "turn off" => board.turn_off(left_corner, right_corner),
            "toggle" => board.toggle(left_corner, right_corner),
            _ => panic!("Invalid command"),
        }
    }

    println!("{}", board.count_lit_lights());
}

struct BoardExtended {
    lights: [[i32; SIZE]; SIZE],
}

impl BoardExtended {
    fn new() -> BoardExtended {
        BoardExtended {
            lights: [[0; SIZE]; SIZE],
        }
    }

    fn turn_on(&mut self, left_corner: (i32, i32), right_corner: (i32, i32)) {
        for x in left_corner.0..=right_corner.0 {
            for y in left_corner.1..=right_corner.1 {
                self.lights[x as usize][y as usize] += 1;
            }
        }
    }

    fn turn_off(&mut self, left_corner: (i32, i32), right_corner: (i32, i32)) {
        for x in left_corner.0..=right_corner.0 {
            for y in left_corner.1..=right_corner.1 {
                self.lights[x as usize][y as usize] =
                    i32::max(self.lights[x as usize][y as usize] - 1, 0);
            }
        }
    }

    fn toggle(&mut self, left_corner: (i32, i32), right_corner: (i32, i32)) {
        for x in left_corner.0..=right_corner.0 {
            for y in left_corner.1..=right_corner.1 {
                self.lights[x as usize][y as usize] += 2;
            }
        }
    }

    fn total_brightness(&self) -> i32 {
        self.lights.iter().flatten().sum()
    }
}

fn part2(input: String) {
    let mut board = BoardExtended::new();
    let instructions = parse_instructions(&input);

    for (command, left_corner, right_corner) in instructions {
        match command.as_str() {
            "turn on" => board.turn_on(left_corner, right_corner),
            "turn off" => board.turn_off(left_corner, right_corner),
            "toggle" => board.toggle(left_corner, right_corner),
            _ => panic!("Invalid command"),
        }
    }

    println!("{}", board.total_brightness());
}
