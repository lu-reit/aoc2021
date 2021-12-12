use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point { 
    pub x: usize,
    pub y: usize
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub maxx: usize,
    pub maxy: usize,
    pub grid: Vec<Vec<u8>>
}

impl Grid {
    fn inc_all(&mut self) -> Vec<Point> {
        let mut flashing: Vec<Point> = Vec::new();
        for y in 0..self.maxy {
             for x in 0..self.maxx {
                 let cycle: &mut u8 = &mut self.grid[y][x];
                 *cycle += 1;
                 if *cycle == 10 {
                     flashing.push( Point { x, y });
                 }
             }
        }
        flashing
    }
    
    fn get_neighbours(&self, pt: &Point) -> Vec<Point> {
        let mut nb: Vec<Point> = Vec::new();
        let (upper, lower, left, right) = (pt.y > 0, pt.y < self.maxy - 1,
            pt.x > 0, pt.x < self.maxx - 1);
        if upper {
            nb.push(Point { x: pt.x, y: pt.y - 1 });
            if left { nb.push(Point { x: pt.x - 1, y: pt.y - 1 })}
            if right { nb.push(Point { x: pt.x + 1, y: pt.y - 1 })}
        }
        if lower {
            nb.push(Point { x: pt.x, y: pt.y + 1 });
            if left { nb.push(Point { x: pt.x - 1, y: pt.y + 1 })}
            if right { nb.push(Point { x: pt.x + 1, y: pt.y + 1 })}
        }
        if left { nb.push(Point { x: pt.x - 1, y: pt.y })}
        if right { nb.push(Point { x: pt.x + 1, y: pt.y })}
        nb
    }

    fn inc_one(&mut self, pt: &Point) -> bool {
        let to_inc: &mut u8 = &mut self.grid[pt.y][pt.x];
        *to_inc += 1;
        if *to_inc == 10 { true } else { false }
    }
    
    fn reset_flashed(&mut self, points: &[Point]) {
        for pt in points.iter() {
            self.grid[pt.y][pt.x] = 0;
        }
    }

    fn all_flashed(&self) -> bool {
        self.grid.iter()
            .all(|row| row.iter().all(|x| *x == 0))
    }
}
 
pub fn part1(input: &Grid) -> usize {
    let mut input_cp: Grid = input.clone();
    let mut flashes: usize = 0;
    for _step_i in 0..100 {
        flashes += step(&mut input_cp);
    }
    flashes
}

pub fn part2(input: &Grid) -> usize {
    let mut input_cp: Grid = input.clone();
    let mut step_i: usize = 1;
    loop { 
        step(&mut input_cp);
        if input_cp.all_flashed() { break; }
        step_i += 1;
    }
    step_i
}

fn step(input: &mut Grid) -> usize {
    let mut all_flashes: Vec<Point> = input.inc_all();
    let mut to_check: Vec<Point> = all_flashes.iter()
        .cloned().collect();
    while !to_check.is_empty() {
        let current = to_check.pop().unwrap();
        let neighbours = input.get_neighbours(&current); 
        for nbx in neighbours.iter() {
            if !all_flashes.contains(nbx) && input.inc_one(nbx) {
                all_flashes.push(*nbx);
                to_check.push(*nbx);
            }
        }
    }
    input.reset_flashed(&all_flashes);
    all_flashes.len()
}

pub fn read_input(fname: &str) -> Grid {
    let file = File::open(fname)
        .expect("File not found");
    let breader = BufReader::new(file);
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line_res in breader.lines() {
        let line: Vec<u8> = line_res.unwrap()
            .trim()
            .as_bytes()
            .iter()
            .map(|x| x - 48)
            .collect();
        grid.push(line);
    }
    let maxx = grid[0].len();
    let maxy = grid.len();
    Grid { maxx, maxy, grid }
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
