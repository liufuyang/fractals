use std::time::Duration;

use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newton_factal::{Field, newton_method_field};
use newton_factal::math::complex::Complex;
use newton_factal::math::polynomial::Polynomial;
use newton_factal::rendering::render_image;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(10));
    for size in [32, 64, 128, 256, 512] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let pol = Polynomial::new(vec![3, 5, 0, 7, -3, 2]); // y = 2x^5 - 3x^4 + 7x^3 + 5x + 3
                let field = Field {
                    source: Complex { re: -5., im: -5. },
                    grid: size,
                    size: 10.0,
                };
                let solutions = newton_method_field(&pol, &field, 100);
                render_image(&black_box(solutions), &black_box(field), 100);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
