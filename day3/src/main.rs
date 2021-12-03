use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = read_input();
    println!("Solution of part1: {}", part1(&input));
    println!("Solution of part2: {}", part2(&input));
}

fn part1(input: &[u32]) -> u32 {
    // Holds the frequencies of 1's at each bit-index
    let mut freq: [u32; 12] = [0; 12];
    for bstr in input {
        for (i, fx) in freq.iter_mut().rev().enumerate() {
            // Right shift the ith-bit to the lsb and isolate it
            *fx += (bstr >> i) & 1;
        }
    }
    let half = (input.len() / 2) as u32;
    let gr = freq.iter().rev().enumerate()
        .fold(0, |sum, (i, fx)| 
            if fx >= &half { sum + (1 << i) }
            else { sum }  
            );
   gr * (gr ^ 0xFFF)
}

fn part2(input: &[u32]) -> u32 {
    let og_rating = find_rating(input, 0);
    let co2s_rating = find_rating(input, 1);
    og_rating * co2s_rating 
}

fn find_rating(input: &[u32], mode: u32) -> u32 {
    let mut cand: Vec<u32> = input.iter().cloned().collect();
    for bit_i in (0u32..12u32).rev() {
        if cand.len() == 1 { break } 

        // Find the frequency of 1 at bit-index bit_i
        let freq = cand.iter().fold(0, |sum, bx| sum + ((bx >> bit_i) & 1));
        let test_bit = (if freq >= (cand.len() as u32 - freq) { 1 } else { 0 }) ^ mode;

        cand = cand.into_iter().filter(|bx| (*bx >> bit_i) & 1 == test_bit).collect();
    }
    cand[0]
}

fn read_input() -> Vec<u32> {
    let fname = get_filename();
     
    let file = File::open(fname).expect("File does not exist");
    let breader = BufReader::new(file);
    breader.lines()
        .map(|line| u32::from_str_radix(line.unwrap().trim(), 2).unwrap())
        .collect()
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
