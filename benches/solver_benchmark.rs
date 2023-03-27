use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasm_sudoku::solve_from_array;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input: Vec<u32> = "300080000000700005100000000000000360002004000070000000000060130045200000000000800"
        .chars().into_iter().map(|c| c.to_digit(10).unwrap()).collect();
    c.bench_function("solve_from_array", |b| {
        b.iter(|| solve_from_array(black_box(input.clone())))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(50);
    targets = criterion_benchmark);
criterion_main!(benches);