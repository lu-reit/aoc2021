use std::env;
use std::fs;
use std::cmp::{min, max};

type Scalar = isize;

pub fn part1(input: &[Scalar]) -> isize {
    let mut sorted: Vec<isize> = input.iter().cloned().collect();
    sorted.sort();
    let median = sorted[(input.len() + 1) / 2];
    sorted.iter().map(|x| (median - x).abs()).sum()
}

pub fn part2(input: &[Scalar]) -> isize {
    let l_min = *(input.iter().min().unwrap());
    let l_max = *(input.iter().max().unwrap());
    let mut min_f_cost = isize::MAX;

    for line in l_min..=l_max {
        let f_cost: isize = input.iter()
            .map(|x| f_distance(*x, line))
            .sum();
        min_f_cost = min(min_f_cost, f_cost);
    }
    min_f_cost
}

pub fn f_distance(x: isize, y: isize) -> isize {
    let dis = (x - y).abs();
    (dis * (dis + 1)) / 2
}

pub fn read_input(fname: &str) -> Vec<Scalar> {
    let data_str = fs::read_to_string(fname)
        .expect("Failed to read file");
    data_str.trim()
        .split(",")
        .map(|num_str| num_str.parse().unwrap())
        .collect()
}
 

pub fn get_filename() -> String {
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
