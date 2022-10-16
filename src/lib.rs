pub mod math;
use math::polynomial::Polynomial;
use math::complex::Complex;
use std::ops::Range;
use colors_transform::{Hsl, Color};
use image::{RgbImage, Rgb};
use itertools::{Itertools, Product};
use std::f32::consts::PI;

pub fn newton_method_approximate(pol: &Polynomial, point: Complex, max_iter: u32) -> (Complex, u32) {
    let tolerance = f64::powi(10.0, -6);
    let dpol = pol.derivative();

    let mut iter = 0;
    let mut diff = 10.0;
    let mut guess = point;

    while iter < max_iter && diff > tolerance {
        let val = pol.evaluate(&guess);
        let der = dpol.evaluate(&guess);
        let div = val / der;

        let next = guess - div;
        let dist = next.euclidean_distance(&guess);
        guess = next;
        diff = dist;

        iter += 1;
    }

    return (guess, iter);
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
        return (x_range).cartesian_product(y_range);
    }

    fn project(&self, spoint: (u32, u32)) -> (f64, f64) {
        let scale = self.tsize / self.ssize as f64;
        (
                self.target.0 + (spoint.0 as f64) * scale,
                self.target.1 + (spoint.1 as f64) * scale
        )
    }
}

fn clamp01(v: f64) -> f64 {
    f64::min(f64::max(0., v), 1.)
}

fn hsl_to_rgb(color: Hsl) -> Rgb<u8> {
    Rgb([
        (color.get_red() ) as u8,
        (color.get_green() ) as u8,
        (color.get_blue() ) as u8,
        ])
}

fn color_from_root(root: Complex, iter: u32, max_iter: u32) -> Hsl {
    let hue = clamp01(f64::abs(0.5 - root.arg() / (PI * 2.))) * 360.;
    let sat = clamp01(f64::abs(0.5 / root.abs())) * 100.;
    let lum = ((max_iter - iter) as f32) / (max_iter as f32) * 100.;
    Hsl::from(hue as f32, sat as f32, lum as f32)
}

pub fn render_image(pol: Polynomial, field: Field) -> RgbImage {
    let mut image = RgbImage::new(field.ssize, field.ssize);
    let max_iter = 100;
    for (i, j) in field.iterate() {
        let (re, im) = field.project((i, j));
        let point = Complex { re, im };
        let (root, iter) = newton_method_approximate(&pol, point, max_iter);
        let color = color_from_root(root, iter, max_iter);
        image.put_pixel(i, j, hsl_to_rgb(color));
    }

    return image;
}