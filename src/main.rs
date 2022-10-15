use std::ops::Range;
use colors_transform::{Hsl, Color};
use image::{RgbImage, Rgb};
use itertools::{Itertools, Product};
use newton_factal::math::complex::Complex;
use newton_factal::math::polynomial::Polynomial;
use newton_factal::newton_method_approximate;
use std::f64::consts::PI;

struct Field {
    source: (u32, u32),
    ssize: u32,

    target: (f64, f64),
    tsize: f64,
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

struct RootPoint {
    root: Complex,
    iter: u32
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

fn color_from_root(roots: &Vec<Complex>, root: RootPoint) -> Hsl {
    for r in roots {
        if *r == root.root {
            let hue = clamp01(f64::abs(0.5 - root.root.arg() / (PI * 2.))) * 360.;
            let sat = clamp01(f64::abs(0.59 / root.root.abs())) * 100.;
            let lum = 0.95 * f64::max(1.0 - root.iter as f64 * 0.025, 0.05) * 100.;
            return Hsl::from(hue as f32, sat as f32, lum as f32);
        }
    }

    Hsl::from(0., 0., 0.)
}

fn render_image(pol: Polynomial, field: Field) -> RgbImage {
    let mut image = RgbImage::new(field.ssize, field.ssize);
    let mut roots = vec![];
    for (i, j) in field.iterate() {
        let (re, im) = field.project((i, j));
        let point = Complex { re, im };
        let (root, iter) = newton_method_approximate(&pol, point, 100);
        if !roots.contains(&root) {
            roots.push(root);
        }
        let rp = RootPoint { root, iter };
        let color = color_from_root(&roots, rp);
        image.put_pixel(i, j, hsl_to_rgb(color));
    }

    return image;
}

fn main() {
    let pol = Polynomial::new(vec![-1, 0, 0, 1]);
    let field = Field {
        source: (0, 0),
        ssize: 512,

        target: (-5., -5.),
        tsize: 10.0,
    };
    let image = render_image(pol, field);
    image.save("output.png").unwrap();
}