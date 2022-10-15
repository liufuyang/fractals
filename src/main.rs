use std::ops::Range;

use image::{RgbImage, Rgb};
use indicatif::ProgressBar;
use itertools::{Itertools, Product};
use newton_factal::math::complex::Complex;
use newton_factal::math::polynomial::Polynomial;
use newton_factal::newton_method_approximate;


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


fn main() {
    let pol = Polynomial::new(vec![-1, 0, 0, 1]);
    let colors = [
        Rgb([0, 0, 0]),
        Rgb([255, 0, 0]),
        Rgb([0, 255, 0]),
        Rgb([0, 0, 255]),
        Rgb([255, 255, 0]),
        Rgb([255, 0, 255]),
        Rgb([0, 255, 255]),
        Rgb([255, 255, 255]),
        Rgb([155, 155, 155]),
        Rgb([55, 55, 55]),
    ];

    let w: u32 = 512;
    let pb = ProgressBar::new(w as u64 * w as u64);
    let mut image = RgbImage::new(w, w);

    let field = Field {
        source: (0, 0),
        ssize: w,

        target: (-0.5, -0.5),
        tsize: 5.0,
    };

    for (i, j) in field.iterate() {
        let (re, im) = field.project((i, j));
        let point = Complex { re, im };
        let (root, iter) = newton_method_approximate(&pol, point, 9);
        let color = colors[(iter % colors.len() as u32) as usize];
        image.put_pixel(i as u32, j as u32, color);
        pb.inc(1);
    }
    image.save("output.png").unwrap();
}
