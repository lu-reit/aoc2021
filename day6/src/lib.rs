use std::env;
use std::fs;

type Scalar = u16;

pub fn simulate(initial: &[Scalar], n_days: usize) -> usize {
    let mut cycles: [usize; 9] = [0; 9];
    for x in initial.iter() {
        cycles[*x as usize] += 1;
    }
    for _day in 0..n_days {
        let spawner = cycles[0];
        for i in 0..8 {
            cycles[i] = cycles[i + 1];
        }
        cycles[6] += spawner;
        cycles[8] = spawner;
    }
    cycles.iter().sum()
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
