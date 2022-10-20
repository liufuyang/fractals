pub mod math;
use image::{Rgb, RgbImage};
use itertools::{Itertools, Product};
use math::complex::Complex;
use math::polynomial::Polynomial;
use std::f32::consts::PI;
use std::ops::Range;

pub fn newton_method_approximate(
    pol: &Polynomial,
    dpol: &Polynomial,
    point: Complex,
    max_iter: u32,
) -> (Complex, u32) {
    let tolerance = f64::powi(10.0, -6);

    let mut iter = 0;
    let mut diff = 10.0;
    let mut guess = point;

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

    (guess, iter)
}

// kind of equlidean distance, just without sqrt
// because we only use the result for termination
// check in comparion with TOLERANCE
fn distance(c1: Complex, c2: Complex) -> f64 {
    (c1.re - c2.re).powi(2) + (c1.im - c2.im).powi(2)
}

pub struct Field {
    pub source: (u32, u32),
    pub ssize: u32,

    pub target: (f64, f64),
    pub tsize: f64,
}

impl Field {
    fn iterate(&self) -> Product<Range<u32>, Range<u32>> {
        let x_range = self.source.0..(self.source.0 + self.ssize);
        let y_range = self.source.1..(self.source.1 + self.ssize);

        (x_range).cartesian_product(y_range)
    }

    fn project(&self, spoint: (u32, u32)) -> (f64, f64) {
        let scale = self.tsize / self.ssize as f64;
        (
            self.target.0 + (spoint.0 as f64) * scale,
            self.target.1 + (spoint.1 as f64) * scale,
        )
    }
}

fn clamp01(v: f32) -> f32 {
    f32::min(f32::max(0., v), 1.)
}

fn color_from_root(root: Complex, iter: u32, max_iter: u32) -> Rgb<u8> {
    let iter = iter as f32;
    let max_iter = max_iter as f32;
    let arg = root.arg() as f32;
    let abs = root.abs() as f32;
    let hue = clamp01(f32::abs(0.5 - arg / (PI * 2.)));
    let sat = clamp01(f32::abs(0.5 / abs));
    let lum = clamp01(f32::abs(0.5 - iter / max_iter));
    let (r, g, b) = hsl_to_rgb(hue, sat, lum);
    Rgb([(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8])
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let q = if l < 0.5 {
        l * (1. + s)
    } else {
        (l + s) - (s * l)
    };

    let p = 2. * l - q;

    let r = f32::max(0., hue_to_rgb(p, q, h + (1. / 3.)));
    let g = f32::max(0., hue_to_rgb(p, q, h));
    let b = f32::max(0., hue_to_rgb(p, q, h - (1. / 3.)));

    (r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, h: f32) -> f32 {
    let h = match h {
        h if h < 0. => h + 1.,
        h if h > 1. => h - 1.,
        _ => h,
    };

    match h {
        h if h < 1. / 6. => p + ((q - p) * 6. * h),
        h if h < 1. / 2. => q,
        h if h < 2. / 3. => p + ((q - p) * 6. * ((2. / 3.) - h)),
        _ => p,
    }
}

pub fn render_image(pol: Polynomial, field: Field) -> RgbImage {
    let mut image = RgbImage::new(field.ssize, field.ssize);
    let max_iter = 100;
    let dpol = pol.derivative();
    for (i, j) in field.iterate() {
        let (re, im) = field.project((i, j));
        let point = Complex { re, im };
        let (root, iter) = newton_method_approximate(&pol, &dpol, point, max_iter);
        let color = color_from_root(root, iter, max_iter);
        image.put_pixel(i, j, color);
    }

    image
}
