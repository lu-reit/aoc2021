use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

pub struct Grid {
    pub maxx: usize,
    pub maxy: usize,
    pub grid: Vec<Vec<u8>>
}

impl Grid {
    pub fn get_height(&self, pt: &Point) -> u8 {
        self.grid[pt.y][pt.x]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
   pub x: usize,
   pub y: usize
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn solve(input: &Grid) -> (usize, usize) {
    let local_minima = get_local_minima(&input);
    let p1_sol = local_minima.iter()
        .fold(0usize, |acc, pt| acc + (input.get_height(&pt) as usize) + 1);

    let mut basin_sizes: Vec<usize> = local_minima.iter()
        .map(|minima| get_basin_size(*minima, input))
        .collect();
    basin_sizes.sort_by(|x, y| y.cmp(x));
    let p2_sol = basin_sizes.iter().take(3)
        .fold(1, |prod, x| prod * x);

    (p1_sol, p2_sol)
}

fn get_basin_size(minima: Point, input: &Grid) -> usize {
    let mut queue: VecDeque<Point> = VecDeque::from([minima]);
    let mut seen: Vec<Point> = vec![minima];
    while !queue.is_empty() {
        let current_pt = queue.pop_front()
            .expect("Basin_size: encountered empty queue");
        let neighbours = get_neighbours(current_pt, input);
         
        for nb in neighbours.iter() {
            if input.get_height(nb) < 9 && !seen.contains(nb) {
                seen.push(*nb);
                queue.push_back(*nb)
            }
        }
    }
    seen.len()
}

fn get_local_minima(input: &Grid) -> Vec<Point> {
    let mut minima: Vec<Point> = Vec::new();
    for y in 0..input.maxy {
        for x in 0..input.maxx {
            let cur_pt = Point { x, y };
            let cur_height = input.get_height(&cur_pt);
            let nb = get_neighbours(cur_pt, input);
            if nb.iter().all(|pt| input.get_height(pt) > cur_height) {
                minima.push(Point { x, y });
            }
        }
    }
    minima
}

fn get_neighbours(pt: Point, grid: &Grid) -> Vec<Point> {
    let mut nb: Vec<Point> = Vec::new();
    if pt.y > 0 { nb.push(Point { x: pt.x, y: pt.y - 1 }); }
    if pt.y < grid.maxy - 1 { nb.push(Point { x: pt.x, y: pt.y + 1}); }
    if pt.x > 0 { nb.push(Point { x: pt.x - 1, y: pt.y }); }
    if pt.x < grid.maxx - 1  { nb.push(Point { x: pt.x + 1, y: pt.y }); }
    nb
}


pub fn read_input(fname: &str) -> Grid {
    let file = File::open(fname)
        .expect("File not found");
    let breader = BufReader::new(file);
    let mut grid: Vec<Vec<u8>>  = Vec::new();

    for line_res in breader.lines() {
        let line = line_res.unwrap();
        let row: Vec<u8> = line.trim()
            .chars()
            .map(|c| c as u8 - 48)
            .collect();
        grid.push(row);
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
