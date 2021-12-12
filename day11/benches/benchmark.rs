use day6::{read_input, part1, part2}; 
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn p1_bench(c: &mut Criterion) {
    let input = read_input("input");

    c.bench_function("part1:", |b| b.iter( 
            || part1(black_box(&input))
    ));
}

fn p2_bench(c: &mut Criterion) {
    let input = read_input("input");

    c.bench_function("part2:", |b| b.iter( 
            || part2(black_box(&input))
    ));
}

fn parse_bench(c: &mut Criterion) {

    c.bench_function("parse:", |b| b.iter( 
            || read_input("input")
    ));
}

criterion_group!(benches, parse_bench, p1_bench, p2_bench);
criterion_main!(benches);

