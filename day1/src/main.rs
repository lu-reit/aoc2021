use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let input = read_input();
    println!("Result of part1: {}", solve(&input, 2));
    println!("Result of part2: {}", solve(&input, 4));
}

fn solve(inp: &[u32], wlen: usize) -> u32 {
    let mut count: u32 = 0;
    for pair in inp.windows(wlen) {
        if pair[0] < pair[wlen - 1] { count += 1}
    }
    count
}

fn read_input() -> Vec<u32> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            panic!("No Argument given. Expecting a filename");
        },
        _ => {
            let file = File::open(&args[1])
                .expect("File does not exist");
            let rdr = BufReader::new(file);
            let mut nums = vec![];
            for line in rdr.lines() {
                nums.push(line
                    .expect("Couldnt parse line")
                    .trim()
                    .parse()
                    .unwrap());
            };
            nums
        }
    }
}
