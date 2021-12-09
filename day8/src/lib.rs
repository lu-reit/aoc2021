use std::env;
use std::fs::File;
use std::cmp::{min, max};
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::HashSet;

// Using Vec instead of fixed sized arrays for simplicity
#[derive(Debug)]
pub struct SegDisplay {
    pub signals: Vec<HashSet<char>>,
    pub digits: Vec<HashSet<char>>
}

pub fn part1(input: &[SegDisplay]) -> usize {
    input.iter()
        .map(|seg_dis| seg_dis.digits.iter()
            .filter(|s| is_unique(s))
            .count())
        .sum()
}

fn is_unique(x: &HashSet<char>) -> bool {
    let n = x.len();
    n == 2 || n == 4 || n == 3 || n == 7
}
    
 
pub fn part2(input: &[SegDisplay]) -> usize {
    let mut sum: usize = 0; 
    for display in input.iter() {
        sum += decode(display);
    }
    
    sum
}

fn decode(display: &SegDisplay) -> usize {
    let empty: HashSet<char> = HashSet::new();
    let mut mapping: Vec<&HashSet<char>> = vec![&empty; 10];
    let eight = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
    mapping[8] = &eight;
    
    for signal in display.signals.iter() {
        let n = signal.len();
        if n == 2 { mapping[1] = signal; }
        else if n == 4 { mapping[4] = signal; }
        else if n == 3 { mapping[7] = signal; }
    }
    for signal in display.signals.iter() {
        if signal.len() == 5 {
            let four_isec: HashSet<char> = mapping[4].intersection(signal)
                .map(|c| *c).collect();

            if four_isec.len() == 2 {
                mapping[2] = signal;
            } else {
                let seven_isec: HashSet<char> = four_isec.intersection(mapping[7])
                    .map(|c| *c).collect();
                if seven_isec.len() == 1 { mapping[5] = signal; }
                else { mapping[3] = signal; }
            }
        }
    }
    let nine = mapping[5].union(mapping[3])
        .map(|c| *c).collect();
    mapping[9] = &nine;
    let leftlower: HashSet<char> = mapping[8].difference(mapping[9])
        .map(|c| *c).collect();
    let six: HashSet<char> = mapping[5].union(&leftlower)
        .map(|c| *c).collect();
    mapping[6] = &six;

    let mut output: usize = 0;
    for (i, set) in display.digits.iter().rev().enumerate() {
        match mapping.iter().position(|&signal| set == signal) {
            Some(pos) => { output += pos * 10usize.pow(i as u32); } 
            None => {}
        }
    }
    output
}


pub fn read_input(fname: &str) -> Vec<SegDisplay> {
    let file = File::open(fname)
        .expect("File not found");
    let breader = BufReader::new(file);
    let mut input: Vec<SegDisplay> = Vec::new();

    for line_res in breader.lines() {
        let line = line_res.unwrap();
        let mut parts = line.split("|");
        let signals = str_to_charset(parts.next().unwrap());
        let digits = str_to_charset(parts.next().unwrap());
        input.push(SegDisplay { signals, digits });
    }
    input
}

fn str_to_charset(string: &str) -> Vec<HashSet<char>> {
    string.trim()
        .split(" ")
        .map(|s| s.chars().collect::<HashSet<char>>())
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
