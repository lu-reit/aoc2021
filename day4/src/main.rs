use std::env;
use std::fs;
use std::collections::HashMap;
use std::time::Instant;


type Field = HashMap<usize, (usize, usize)>;
type CheckMap = [usize; 5];

struct Input {
    draw_nums: Vec<usize>,
    fields: Vec<Field>
}

fn main() {
    let input_timer = Instant::now();
    let input = read_input(); 
    println!("Elapsed time for parsing: {:?}", input_timer.elapsed());

    let p1_timer = Instant::now();
    let p1_sol = part1(&input);
    let p1_time = p1_timer.elapsed();
    println!("Solution of part1: {}", p1_sol);
    println!("Elapsed time for part 1: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_sol = part2(&input);
    let p2_time = p2_timer.elapsed();
    println!("Solution of part2: {}", p2_sol);
    println!("Elapsed time for part 2: {:?}", p2_time);
}

fn part1(input: &Input) -> usize {
    let mut checked: Vec<CheckMap> = vec![[0; 5]; input.fields.len()];

    for draw in input.draw_nums.iter() {
        for (i, field) in input.fields.iter().enumerate() {
            match field.get(draw) {
                Some((x, y)) => {
                    checked[i][*y] ^= 1 << x;
                    if test_for_win(&checked[i]) { 
                        return *draw * calc_score(field, &checked[i]) 
                    }
                }
                None => {}
            }
        }
    }
    panic!("Somebody must win");
}

fn part2(input: &Input) -> usize {
    let mut checked: Vec<CheckMap> = vec![ [0; 5]; input.fields.len()];
    let mut has_won: Vec<bool> = vec![false; input.fields.len()];
    let mut last_winner: usize = 0;
    let mut last_win_draw: usize = 0;

    for draw in input.draw_nums.iter() {
        for (i, field) in input.fields.iter().enumerate() {
            if has_won[i] { continue }

            match field.get(draw) {
                Some((x, y)) => {
                    checked[i][*y] ^= 1 << x;
                    if test_for_win(&checked[i]) { 
                        has_won[i] = true;
                        last_winner = i;
                        last_win_draw = *draw;
                    }
                }
                None => {}
            }
        }
    }
    last_win_draw * calc_score(&input.fields[last_winner], &checked[last_winner])
}

fn test_for_win(checkmap: &CheckMap) -> bool {
    // First test the rows
    if checkmap.iter().any(|row| *row == 0b11111) { return true }

    // Next the columns
    checkmap.iter().fold(0b11111, |sum, row| sum & row) > 0
}

fn calc_score(field: &Field, checkmap: &CheckMap) -> usize {
    field.iter().fold(0, |sum, (num, (x, y))| 
        if checkmap[*y] & (1 << x) == 0 { sum + num } else { sum })
}
    
fn read_input() -> Input {
    let fname = get_filename();
    let data_str = fs::read_to_string(fname)
        .expect("Failed to read file");
    let mut linebreak2 = data_str.split("\n\n");

    let draw_nums: Vec<usize> = linebreak2.next().unwrap()
        .split(",")
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let mut fields: Vec<Field> = Vec::new();
    for block in linebreak2 {
        let field: Field = block.split_whitespace()
            .enumerate() 
            .map(|(i, num)| (num.parse().unwrap(), (i / 5, i % 5)))
            .collect();
        fields.push(field);
    }
    Input { draw_nums, fields }
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
