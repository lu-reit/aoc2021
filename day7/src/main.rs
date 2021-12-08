use day6::{read_input, part1, part2, get_filename}; 

fn main() {
    let fname = get_filename();
    let input = read_input(&fname); 

    let p1_sol = part1(&input);
    let p2_sol = part2(&input);
    println!("Solution for part1: {}", p1_sol);
    println!("Solution for part2: {}", p2_sol);
}
