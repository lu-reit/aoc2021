use std::env;
use std::fs;
use std::collections::HashMap;

type Field = HashMap<usize, (usize, usize)>;
type CheckMap = [[bool; 5]; 5];

struct Input {
    draw_nums: Vec<usize>,
    fields: Vec<Field>
}

fn main() {
    let input = read_input(); 
    println!("Solution of part1: {}", part1(&input));
    println!("Solution of part2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let mut checked: Vec<CheckMap> = vec![ [[false; 5]; 5]; input.fields.len()];

    for draw in input.draw_nums.iter() {
        for (i, field) in input.fields.iter().enumerate() {
            match field.get(draw) {
                Some((x, y)) => {
                    checked[i][*x][*y] = true;
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
    let mut checked: Vec<CheckMap> = vec![ [[false; 5]; 5]; input.fields.len()];
    let mut has_won: Vec<bool> = vec![false; input.fields.len()];
    let mut last_winner: usize = 0;
    let mut last_win_draw: usize = 0;

    for draw in input.draw_nums.iter() {
        for (i, field) in input.fields.iter().enumerate() {
            if has_won[i] { continue }

            match field.get(draw) {
                Some((x, y)) => {
                    checked[i][*x][*y] = true;
                    if test_for_win(&checked[i]) { 
                        has_won[i] = true;
                        last_winner = i;
                        last_win_draw = draw;
                    }
                }
                None => {}
            }
        }
    }
    last_win_draw * calc_score(fields[i], checked[i])
}

fn test_for_win(checkmap: &CheckMap) -> bool {
    // First test the rows
    if checkmap.iter().any(|row| *row == [true, true, true, true, true]) { return true }

    // Next the columns
    'outer: for i in 0..5 {
        for j in 0..5 {
            if !checkmap[j][i] { continue 'outer }
        }
        return true;
    }
    return false;
}

fn calc_score(field: &Field, checkmap: &CheckMap) -> usize {
    field.iter().fold(0, |sum, (num, (x, y))| 
        if !checkmap[*x][*y] { sum + num } else { sum })
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
