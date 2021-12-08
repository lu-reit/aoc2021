use day6::{read_input, part1, part2}; 
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn p1_bench(c: &mut Criterion) {
    let input = read_input("input");

    c.bench_function("part1: ", |b| b.iter( 
            || part1(black_box(&input))
    ));
}

fn p2_bench(c: &mut Criterion) {
    let input = read_input("input");

    c.bench_function("part2: 256 cycles", |b| b.iter( 
            || part2(black_box(&input))
    ));
}

criterion_group!(benches, p1_bench, p2_bench);
criterion_main!(benches);

