use day6::{read_input, simulate, get_filename}; 

fn main() {
    let fname = get_filename();
    let input = read_input(&fname); 

    let p1_sol = simulate(&input, 80);
    let p2_sol = simulate(&input, 256);

    println!("Solution for part1: {}", p1_sol);
    println!("Solution for part2: {}", p2_sol);
}
