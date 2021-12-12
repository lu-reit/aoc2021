use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Input = Vec<Vec<u8>>;
type Scalar = u64;

const PAIRS: [(u8, u8); 4] = [(b'(', b')'), (b'[', b']'), 
      (b'{', b'}'), (b'<', b'>')];
const SCORES: [Scalar; 4] = [3, 57, 1197, 25137];

pub fn bytes_to_string(bytes: &[u8]) -> String {
    std::str::from_utf8(bytes).unwrap().to_string()
}

pub fn solve(input: &Input) -> (Scalar, Scalar) {
    let scores: Vec<(Scalar, Scalar)> = input.iter()
        .map(|row| line_score(row))
        .collect();
    
    let p1_score = scores.iter()
        .fold(0, |sum, score| sum + score.0);

    let mut incomplete: Vec<Scalar> = scores.iter()
        .filter(|(p1, _p2)| *p1 == 0)
        .map(|(_p1, p2)| *p2)
        .collect();
    incomplete.sort();
    let p2_score = incomplete[incomplete.len() / 2];

    (p1_score, p2_score)
}

fn line_score(bytes: &[u8]) -> (Scalar, Scalar) {
    let mut stack: Vec<u8> = Vec::new();
    for byte in bytes {
        match byte {
            b'(' | b'[' | b'{' | b'<' => {
                stack.push(*byte);
            }
            _ => {
                let previous = stack.pop().unwrap();
                match PAIRS.iter().position(|x| x.1 == *byte) {
                    Some(i) => {
                        if PAIRS[i].0 != previous { return (SCORES[i], 0) }
                    }
                    None => panic!("line_score: unknown character"),
                }
            }
        };
    }
    let incomplete_score = stack.iter().rev()
        .fold(0, |sum, byte| {
            let i = PAIRS.iter().position(|x| x.0 == *byte).unwrap();
            sum * 5 + (i + 1) as Scalar 
        });
    (0, incomplete_score)
}

pub fn read_input(fname: &str) -> Input {
    let file = File::open(fname)
        .expect("File not found");
    let breader = BufReader::new(file);
    let mut input: Input = Vec::new();

    for line_res in breader.lines() {
        let line = line_res.unwrap().trim().to_string();
        input.push(line.into_bytes());
    }
    input
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
