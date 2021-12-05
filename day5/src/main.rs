use std::env;
use std::fs;
use std::time::Instant;
use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(Debug)]
struct Input {
    maxx: usize,
    maxy: usize,
    hori: Vec<Line>,
    vert: Vec<Line>,
    diag: Vec<Line>
}

fn main() {
    let input_timer = Instant::now();
    let input = read_input(); 
    println!("Elapsed time for parsing: {:?}", input_timer.elapsed());

    let solve_timer = Instant::now();
    let (p1_sol, p2_sol) = solve(&input);
    let solve_time = solve_timer.elapsed();
    println!("Solution of part1 and part2: ({}, {})", p1_sol, p2_sol);
    println!("Elapsed time for both parts: {:?}", solve_time);
}

fn solve(input: &Input) -> (usize, usize) {
    let mut grid: Vec<Vec<usize>> = vec![vec![0; input.maxx + 1]; input.maxy + 1];

    for hline in input.hori.iter() {
        for x in hline.x1..=hline.x2 {
            grid[hline.y1][x] += 1;
        }
    }
    for vline in input.vert.iter() {
        for y in vline.y1..=vline.y2 {
            grid[y][vline.x1] += 1;
        }
    }
    let p1_overlaps = get_overlaps(&grid);

    for dline in input.diag.iter() {
        let dx: isize = (dline.x2 as isize - dline.x1 as isize).signum();
        let dy: isize = (dline.y2 as isize - dline.y1 as isize).signum();
        let mut cx: usize = dline.x1;
        let mut cy: usize = dline.y1;
        loop {
            grid[cy][cx] += 1;
            cx = (cx as isize + dx) as usize;
            cy = (cy as isize + dy) as usize;
            if cx == dline.x2 && cy == dline.y2 { 
                grid[cy][cx] += 1;
                break 
            }
        }
    }
    (p1_overlaps, get_overlaps(&grid))
}

fn get_overlaps(grid: &Vec<Vec<usize>>) -> usize {
    let mut overlaps: usize = 0;
    for hline in grid.iter() {
        for n_overlaps in hline.iter() {
            if *n_overlaps > 1 { overlaps += 1; }
        }
    }
    overlaps
}

fn read_input() -> Input {
    let fname = get_filename();
    let mut data_str = fs::read_to_string(fname)
        .expect("Failed to read file");
    data_str = data_str.trim().to_string();
    parse_lines(&data_str)
}

fn parse_lines(data_str: &str) -> Input {
    let re = Regex::new("[0-9]+").unwrap();
    let mut input = Input{ maxx: 0, maxy: 0,
        hori: Vec::new(), vert: Vec::new(), diag: Vec::new() };

    for line_str in data_str.split("\n") {
        let mut matches = re.find_iter(line_str);
        let x1: usize = matches.next().unwrap().as_str().parse().unwrap();
        let y1: usize = matches.next().unwrap().as_str().parse().unwrap();
        let x2: usize = matches.next().unwrap().as_str().parse().unwrap();
        let y2: usize = matches.next().unwrap().as_str().parse().unwrap();
        input.maxx = max(input.maxx, max(x1, x2));
        input.maxy = max(input.maxy, max(y1, y2));
        if x1 == x2 { 
            if y2 > y1 {
                input.vert.push(Line{ x1, y1, x2, y2 });
            } else {
                input.vert.push(Line{ x1: x2, y1: y2, x2: x1, y2: y1 });
            }
        }
        else if y1 == y2 {
            if x2 > x1 {
                input.hori.push(Line{ x1, y1, x2, y2 });
            } else {
                input.hori.push(Line{ x1: x2, y1: y2, x2: x1, y2: y1 });
            }
        } else {
            input.diag.push(Line{ x1, y1, x2, y2 });
        }
    }
    input
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
