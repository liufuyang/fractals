pub mod math;
use image::{Rgb, RgbImage};
use itertools::{Itertools, Product};
use math::color::color_from_root;
use math::complex::Complex;
use math::polynomial::Polynomial;
use std::ops::Range;

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

// kind of equlidean distance, just without sqrt
// because we only use the result for termination
// check in comparion with TOLERANCE
fn distance(c1: Complex, c2: Complex) -> f64 {
    (c1.re - c2.re).powi(2) + (c1.im - c2.im).powi(2)
}

pub struct Field {
    pub source: Complex,
    pub size: f64,
    pub grid: u32,
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

pub struct Solution {
    root: Complex,
    iter: u32,
}

pub fn render_image(pol: Polynomial, field: Field) -> RgbImage {
    let max_iter = 100;
    let dpol = pol.derivative();

    let solutions: Vec<Solution> = field
        .values()
        .iter()
        .map(|point| newton_method_approximate(&pol, &dpol, point, max_iter))
        .collect();

    let mut image = RgbImage::new(field.grid, field.grid);
    let mut iter = solutions.iter();
    for i in 0..field.grid {
        for j in 0..field.grid {
            let solution = iter.next().expect("not enough values in solutions");
            let (r, g, b) = color_from_root(solution.root, solution.iter, max_iter);
            image.put_pixel(i, j, Rgb([r, g, b]));
        }
    }

    image
}
