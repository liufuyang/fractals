
use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newton_factal::math::complex::Complex;
use newton_factal::rendering::render_image;
use newton_factal::{Field, Solution};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");
    for size in [32, 64, 128, 256, 512] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                // todo add seed to make the results comparible
                let mut rng = rand::thread_rng();
                let max_iter = 100;
                let roots: Vec<Complex> = (0..5).map(|_| Complex { re: rng.gen(), im: rng.gen() }).collect();
                let solutions = (0..(size*size))
                    .map(|_| Solution {root: roots[rng.gen_range(0..roots.len())], iter: rng.gen_range(0..max_iter)})
                    .collect();
                let field = Field {
                    source: Complex { re: -5., im: -5. },
                    grid: size,
                    size: 10.0,
                };
                render_image(&black_box(solutions), &black_box(field), 100);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
