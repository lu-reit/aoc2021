use day6::{read_input, simulate}; 
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn p1_bench(c: &mut Criterion) {
    let input = read_input("input");

    c.bench_function("part1: 80 cycles", |b| b.iter( 
            || simulate(black_box(&input), black_box(80))
    ));
}

fn p2_bench(c: &mut Criterion) {
    let input = read_input("input");

    c.bench_function("part2: 256 cycles", |b| b.iter( 
            || simulate(black_box(&input), black_box(256))
    ));
}

criterion_group!(benches, p1_bench, p2_bench);
criterion_main!(benches);

