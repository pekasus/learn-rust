use criterion::{black_box, criterion_group, criterion_main, Criterion};

const ZEROS: [u8; 1] = [0];

pub fn all(c: &mut Criterion) {
    let mut group = c.benchmark_group("is_zero");
    group.bench_function("bench_is_zero_by_byte", |b| {
        b.iter(|| is_zero_by_format(black_box(&ZEROS[..])));
    });

    group.bench_function("bench_is_zero_by_leading_zeros", |b| {
        b.iter(|| is_zero_by_leading_zeros(black_box(&ZEROS[..])));
    });
    group.finish();
}

criterion_group!(all_group, all);

criterion_main!(all_group);

fn is_zero_by_format(bytes: &[u8]) -> bool {
    // dbg!(format!("{:?}", bytes[0]));
    // @TODO format with leading zeros:
    format!("{:?}", bytes).starts_with("00")
}

fn is_zero_by_leading_zeros(bytes: &[u8]) -> bool {
    bytes[0].leading_zeros() == 8
}
