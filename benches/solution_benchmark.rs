use criterion::{black_box, criterion_group, criterion_main, Criterion};
use newton_factal::newton_method_approximate;
use newton_factal::math::polynomial::Polynomial;
use newton_factal::math::complex::Complex;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pol 5", |b| b.iter(|| {
        let pol = Polynomial::new(vec![3, 5, 0, 7, -3, 2]); // y = 2x^5 - 3x^4 + 7x^3 + 5x + 3
        let com = Complex { re: 4., im: 2.};
        newton_method_approximate(black_box(&pol), black_box(com), 9);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);