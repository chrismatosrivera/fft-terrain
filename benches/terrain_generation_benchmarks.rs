use criterion::{black_box, criterion_group, criterion_main, Criterion};

use crate::noise::noise_terrain;
use crate::noise::NoiseType;

mod terrain;
mod noise;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("diamond square n = 8", |b| b.iter(|| terrain::midpoint_displacement(8, 0.75, 0.50, 85.0)));
    c.bench_function("fft n = 8", |b| b.iter(|| terrain::fft_terrain(8)));
    c.bench_function("perlin n = 8", |b| b.iter(|| noise_terrain(8, NoiseType::Perlin)));
    c.bench_function("simplex n = 8", |b| b.iter(|| noise_terrain(8, NoiseType::Simplex)));
    c.bench_function("worley n = 8", |b| b.iter(|| noise_terrain(8, NoiseType::Worley)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
