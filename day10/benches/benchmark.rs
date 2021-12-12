use day6::{read_input, solve}; 
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn solve_bench(c: &mut Criterion) {
    let input = read_input("input");

    c.bench_function("solve:", |b| b.iter( 
            || solve(black_box(&input))
    ));
}

fn parse_bench(c: &mut Criterion) {

    c.bench_function("parse:", |b| b.iter( 
            || read_input("input")
    ));
}

criterion_group!(benches, parse_bench, solve_bench);
criterion_main!(benches);

