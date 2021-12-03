use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    let input = read_input();
    println!("Result of part1: {}", part1(&input));
    println!("Result of part2: {}", part2(&input));
}

fn part1(input: &[Direction]) -> i32 {
    let mut hpos = 0;
    let mut depth = 0;

    for command in input {
        match command {
            Direction::Forward(x) => hpos += x,
            Direction::Down(x) => depth += x,
            Direction::Up(x) => depth -= x
        }
    }
    hpos * depth
}

fn part2(input: &[Direction]) -> i32 {
    let mut hpos = 0;
    let mut aim = 0;
    let mut depth = 0;

    for command in input {
        match command {
            Direction::Forward(x) => {
                hpos += x;
                depth += aim * x
            }
            Direction::Down(x) => aim += x,
            Direction::Up(x) => aim -= x
        }
    }
    hpos * depth
}

fn read_input() -> Vec<Direction> {
    let fname = get_filename();
    let file = File::open(fname).expect("File does not exist");
    let breader = BufReader::new(file);
    let mut input = vec![];

    for line in breader.lines() {
        let u = line.unwrap();
        let mut parts = u.split(' ');
        let dir_str = parts.next().unwrap();
        let mag = parts.next().unwrap().parse::<i32>().unwrap();
        match dir_str {
            "forward" => input.push(Direction::Forward(mag)),
            "up" => input.push(Direction::Up(mag)),
            "down" => input.push(Direction::Down(mag)),
            _ => panic!("Unkown command received")
        }
    }
    input
}

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            panic!("No Argument given. Expecting a filename");
        },
        _ => {
            args[1].to_string()
        }
    }
}
