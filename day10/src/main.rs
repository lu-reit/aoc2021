use day6::{read_input, get_filename, solve}; 

fn main() {
    let fname = get_filename();
    let input = read_input(&fname); 
    let (p1_sol, p2_sol) = solve(&input);

    println!("Solution for part1: {}", p1_sol);
    println!("Solution for part2: {}", p2_sol);
}
