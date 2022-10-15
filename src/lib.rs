pub mod math;
use math::polynomial::Polynomial;
use math::complex::Complex;

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
