use bevy_pixel_buffers::shapes::rounded_rectangle;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rounded_rectangle", |b| {
        b.iter(|| rounded_rectangle(black_box(64), black_box(64), black_box(15), black_box(3)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
