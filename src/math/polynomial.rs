use crate::math::complex::Complex;
use std::fmt;
use itertools::Itertools; // 0.8.0

#[derive(Debug)]
pub struct Polynomial {
    coeff: Vec<i32>
}

impl Polynomial {
    pub fn new(coeff: Vec<i32>) -> Self {
        Polynomial {
            coeff
        }
    }

    pub fn derivative(&self) -> Polynomial {
        if self.coeff.len() <= 1 {
            return Polynomial::new(vec![])
        }

        // todo size hint?
        let mut new_coef: Vec<i32> = Vec::new();
        for i in 0..(self.coeff.len() - 1) {
            let nc: i32 = self.coeff[i+1] * ((i as i32) + 1);
            new_coef.push(nc)
        }

        Polynomial {
            coeff: new_coef
        }
    }

    pub fn evaluate(&self, z: &Complex) -> Complex {
        let mut r = Complex::zero();

        for (index, coef) in self.coeff.iter().enumerate() {
            // todo we can improve the performance of the code by using int powers here
            let p = z.powf(index as f64);
            let pm = p.multiply(*coef as f64);
            r = r + pm
        }

        r
    }

    fn show_part(index: usize, coef: i32) -> String {
        let s_coef = if coef == 1 { String::from("") } else { format!("{}", coef) };
        match  index {
            0 => format!("{}", coef),
            1 => format!("{}x", s_coef),
            _ => format!("{}x^{}", s_coef, index),
        }
    }
}


impl fmt::Display for Polynomial {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res: String = self.coeff.iter().enumerate()
            .filter(|(_, coef)| **coef > 0)
            .map(|(index, coef)| Polynomial::show_part(index, *coef))
            .intersperse(" + ".to_string())
            .collect();

        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derivate_constant() {
        // y = 1
        // y = 1*x^0
        let pol = Polynomial::new(vec![1]);
        assert_eq!(true, pol.derivative().coeff.is_empty());
    }

    #[test]
    fn derivate_linear() {
        // y = x
        // y = 0*x^0 + 1*x^1
        let pol = Polynomial::new(vec![0, 1]);
        assert_eq!(vec![1], pol.derivative().coeff);
    }

    #[test]
    fn derivate_pol2() {
        // y = x ^ 2
        // y = 0*x^0 + 0*x^1 + 1*x^2
        let pol = Polynomial::new(vec![0, 0, 1]);
        assert_eq!(vec![0, 2], pol.derivative().coeff);
    }

    #[test]
    fn derivate_pol5() {
        let pol = Polynomial::new(vec![1, 1, 2, 3, 5, 8]);
        // y = 1 + x + 2x^2 + 3x^3 + 5x^4 + 8x^5
        // y' = 1 + 4x + 9x^2 + 20x^4 + 40x^5
        assert_eq!(vec![1, 4, 9, 20, 40], pol.derivative().coeff);
    }

    #[test]
    fn display_constant() {
        // y = 1
        let pol = Polynomial::new(vec![1]);
        assert_eq!("1", format!("{}", pol));
    }

    #[test]
    fn display_linear() {
        // y = x
        let pol = Polynomial::new(vec![0, 1]);
        assert_eq!("x", format!("{}", pol));
    }

    #[test]
    fn display_pol2() {
        // y = x ^ 2
        let pol = Polynomial::new(vec![0, 0, 1]);
        assert_eq!("x^2", format!("{}", pol));
    }

    #[test]
    fn display_pol5() {
        let pol = Polynomial::new(vec![1, 1, 2, 3, 5, 8]);
        // y = 1 + x + 2x^2 + 3x^3 + 5x^4 + 8x^5
        assert_eq!("1 + x + 2x^2 + 3x^3 + 5x^4 + 8x^5", format!("{}", pol));
    }
}
