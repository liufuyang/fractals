use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newton_factal::{render_image, Field};
use newton_factal::math::polynomial::Polynomial;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("render", |b| b.iter(|| {
        let pol = Polynomial::new(vec![3, 5, 0, 7, -3, 2]); // y = 2x^5 - 3x^4 + 7x^3 + 5x + 3
        let field = Field {
            source: (0, 0),
            ssize: 512,

            target: (-5., -5.),
            tsize: 10.0,
        };
        render_image(black_box(pol), black_box(field));
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);