use image::{Rgb, RgbImage};
use std::f32::consts::PI;

use crate::{Field, Solution};

pub fn render_image(solutions: &Vec<Solution>, field: &Field, max_iter: u32) -> RgbImage {
    let mut image = RgbImage::new(field.grid, field.grid);
    let mut iter = solutions.iter();
    for i in 0..field.grid {
        for j in 0..field.grid {
            let solution = iter.next().expect("not enough values in solutions");
            let (r, g, b) = color_from_root(solution, max_iter);
            image.put_pixel(i, j, Rgb([r, g, b]));
        }
    }

    image
}

fn color_from_root(solution: &Solution, max_iter: u32) -> (u8, u8, u8) {
    let iter = solution.iter as f32;
    let max_iter = max_iter as f32;
    let arg = solution.root.arg() as f32;
    let abs = solution.root.abs() as f32;
    let hue = clamp01(f32::abs(0.5 - arg / (PI * 2.)));
    let sat = clamp01(f32::abs(0.5 / abs));
    let lum = clamp01(f32::abs(0.5 - iter / max_iter));
    let (r, g, b) = hsl_to_rgb(hue, sat, lum);
    ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8)
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

fn clamp01(v: f32) -> f32 {
    f32::min(f32::max(0., v), 1.)
}
