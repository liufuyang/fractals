use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newton_factal::{render_image, Field};
use newton_factal::math::polynomial::Polynomial;
use criterion::BenchmarkId;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("render");
    group.sample_size(10).measurement_time(Duration::from_secs(10));
    for size in [32, 64, 128, 256, 512] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| b.iter(|| {
            let pol = Polynomial::new(vec![3, 5, 0, 7, -3, 2]); // y = 2x^5 - 3x^4 + 7x^3 + 5x + 3
            let field = Field {
                source: (0, 0),
                ssize: size,

                target: (-5., -5.),
                tsize: 10.0,
            };
            render_image(black_box(pol), black_box(field));
        }));
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);