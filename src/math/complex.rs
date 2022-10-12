use std::ops::{Add, Sub, Div};
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64
}

// todo read up on operator overloading in Rust.
// there are some traits like Mul<T> that I saw in the errors
impl Complex {
    pub fn abs(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn powf(&self, p: f64) -> Self {
        let r = self.abs().powf(p);
        let theta = p * self.arg();
        // todo there should be a nicer way of writing math functions
        // I'd want to see cos(theta) intead of theta.cos()
        Complex {
            re: theta.cos() * r,
            im: theta.sin() * r
        }
    }

    pub fn euclidean_distance(&self, other: &Self) -> f64 {
        ((self.re - other.re).powi(2) + (self.im - other.im).powi(2)).sqrt()
    }

    pub fn zero() -> Complex {
        Complex {
            re: 0.,
            im: 0.
        }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im
        }
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let k = rhs.re.powi(2) + rhs.im.powi(2);
        let re = (self.re * rhs.re + self.im * rhs.im) / k;
        let im = (self.im * rhs.re - self.re * rhs.im) / k;

        Self { re, im}
    }
}

impl Complex {
    pub fn multiply(self, f: f64) -> Self {
        Self {
            re: self.re * f,
            im: self.im * f
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}i", self.re, self.im)
    }
}
