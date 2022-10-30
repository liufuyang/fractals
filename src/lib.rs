pub mod math;
pub mod rendering;

use itertools::Itertools;

use math::complex::Complex;
use math::polynomial::Polynomial;

pub struct Field {
    pub source: Complex,
    pub size: f64,
    pub grid: u32,
}
pub struct Solution {
    pub root: Complex,
    pub iter: u32,
}

pub fn newton_method_field(pol: &Polynomial, field: &Field, max_iter: u32) -> Vec<Solution> {
    let dpol = pol.derivative();

    field
        .values()
        .iter()
        .map(|point| newton_method_approximate(&pol, &dpol, point, max_iter))
        .collect()
}

pub fn newton_method_approximate(
    pol: &Polynomial,
    dpol: &Polynomial,
    point: &Complex,
    max_iter: u32,
) -> Solution {
    let tolerance = f64::powi(10.0, -6);

    let mut iter = 0;
    let mut diff = 10.0;
    let mut guess = point.clone();

    while iter < max_iter && diff > tolerance {
        let val = pol.evaluate(&guess);
        let der = dpol.evaluate(&guess);
        let div = val / der;

        let next = guess - div;
        let dist = distance(next, guess);
        guess = next;
        diff = dist;

        iter += 1;
    }

    Solution { root: guess, iter }
}

// euqlidean distance without sqrt.
// sqrt is expensive and I don't need the exact distance here
// since I only use the result for the termination
// check in comparsion with TOLERANCE.
fn distance(c1: Complex, c2: Complex) -> f64 {
    (c1.re - c2.re).powi(2) + (c1.im - c2.im).powi(2)
}

impl Field {
    fn values(&self) -> Vec<Complex> {
        let step = self.size / (self.grid as f64);

        let re_range = (0..self.grid).map(|i| self.source.re + (i as f64) * step);
        let im_range = (0..self.grid).map(|i| self.source.im + (i as f64) * step);

        (re_range)
            .cartesian_product(im_range)
            .map(|(re, im)| Complex { re, im })
            .collect()
    }
}
