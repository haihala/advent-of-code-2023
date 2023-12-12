use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_12::line_permutations;

const EASY_LINE: &str = "???.### 1,1,3";
const MEDIUM_LINE: &str = ".??..??...?##. 1,1,3";
const HARD_LINE: &str = ".#?#???????.????# 1,2,3,2,1";

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Easy line", |b| {
        b.iter(|| {
            line_permutations(black_box(EASY_LINE));
        })
    });

    c.bench_function("Medium line", |b| {
        b.iter(|| {
            line_permutations(black_box(MEDIUM_LINE));
        })
    });

    c.bench_function("Hard line", |b| {
        b.iter(|| {
            line_permutations(black_box(HARD_LINE));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
