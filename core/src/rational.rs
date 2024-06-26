use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Rational {
    p: f64, //numerator
    q: f64, //denominator
}

fn gcd(x: f64, y: f64) -> f64 {
    let mut x = x.abs();
    let mut y = y.abs();
    while y > 0.0 {
        let rem = x % y;
        x = y;
        y = rem;
    }
    x
}

fn lcm(x: f64, y: f64) -> f64 {
    x * y / gcd(x, y)
}

impl Rational {
    pub fn new(a: f64, b: f64) -> Self {
        Self { p: a, q: b }
    }

    pub fn zero() -> Self {
        Self { p: 0.0, q: 1.0 }
    }

    pub fn one() -> Self {
        Self { p: 1.0, q: 1.0 }
    }

    pub fn reduce(&self) -> Self {
        let mut p = self.p;
        let mut q = self.q;
        if q.is_sign_negative() {
            q *= -1.0;
            p *= -1.0;
        }
        let g = gcd(p, q);
        p /= g;
        q /= g;
        Self { p, q }
    }

    //Common denominator
    pub fn com_den(&mut self, mut other: Rational) -> Rational {
        let l = lcm(self.q, other.q);
        self.p *= l / self.q;
        self.q = l;
        other.p *= l / other.q;
        other.q = l;
        other
    }

    pub fn to_float(&self) -> f64 {
        self.p / self.q
    }

    pub fn abs(&self) -> Self {
        Self {
            p: self.p.abs(),
            q: self.q.abs(),
        }
        .reduce()
    }

    pub fn sqrt(&self) -> Self {
        Self {
            p: self.p.sqrt(),
            q: self.q.sqrt(),
        }
        .reduce()
    }

    pub fn powf(&self, x: f64) -> Self {
        Self {
            p: self.p.powf(x),
            q: self.q.powf(x),
        }
        .reduce()
    }

    pub fn pow(&self, x: Rational) -> Self {
        let cp = self.powf(x.to_float());
        Self { p: cp.p, q: cp.q }.reduce()
    }

    pub fn log(&self, x: Rational) -> Self {
        let x = x.to_float();
        Self {
            p: self.p.log(x) - self.q.log(x),
            q: 1.0,
        }
        .reduce()
    }
}

impl std::fmt::Debug for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("{}", self.to_float()))
        let (p, q) = self.reduce().into();
        if q == 0.0 {
            return f.write_str("undefined");
        }
        if q == 1.0 {
            f.write_fmt(format_args!("{}", p))
        } else {
            f.write_fmt(format_args!("{}/{}", p, q))
        }
    }
}
impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("{}", self.to_float()))
        let (p, q) = self.reduce().into();
        if q == 0.0 {
            return f.write_str("undefined");
        }
        if q == 1.0 {
            f.write_fmt(format_args!("{}", p))
        } else {
            f.write_fmt(format_args!("{}/{}", p, q))
        }
    }
}

impl From<f64> for Rational {
    fn from(mut p: f64) -> Rational {
        let sp = p.to_string();
        let sp = sp.split(".").collect::<Vec<_>>();
        let mut q = 1.0;
        if sp.len() == 2 {
            if let Some(last) = sp.last() {
                let dec_places = last.len() as i32;
                let exponent = f64::powi(10.0, dec_places);
                p *= exponent;
                q *= exponent;
            }
        }

        Rational { p, q }.reduce()
    }
}

impl From<f32> for Rational {
    fn from(p: f32) -> Rational {
        Rational::from(f64::from(p))
    }
}

impl From<Rational> for (f64, f64) {
    fn from(r: Rational) -> (f64, f64) {
        (r.p, r.q)
    }
}

impl std::ops::Add<Rational> for Rational {
    type Output = Rational;
    fn add(mut self, other: Rational) -> Rational {
        let p = self.com_den(other).p;
        self.p += p;
        self.reduce()
    }
}

impl std::ops::Sub<Rational> for Rational {
    type Output = Rational;
    fn sub(mut self, other: Rational) -> Rational {
        let p = self.com_den(other).p;
        self.p -= p;
        self.reduce()
    }
}

impl std::ops::Mul<Rational> for Rational {
    type Output = Rational;
    fn mul(mut self, other: Rational) -> Rational {
        self.p *= other.p;
        self.q *= other.q;
        self.reduce()
    }
}

impl std::ops::Div<Rational> for Rational {
    type Output = Rational;
    fn div(mut self, other: Rational) -> Rational {
        self.p *= other.q;
        self.q *= other.p;
        self.reduce()
    }
}

impl std::ops::Neg for Rational {
    type Output = Rational;

    fn neg(mut self) -> Rational {
        self.p = -self.p;
        self
    }
}

impl std::ops::Rem for Rational {
    type Output = Rational;

    fn rem(self, other: Self) -> Self::Output {
        Self {
            p: self.to_float() % other.to_float(),
            q: 1.0,
        }
        .reduce()
    }
}

mod test {
    #[cfg(test)]
    use super::Rational;

    #[test]
    fn reduce() {
        assert_eq!(Rational::new(10.0, -2.0).reduce(), Rational::new(-5.0, 1.0));
        assert_eq!(Rational::new(-2.0, -10.0).reduce(), Rational::new(1.0, 5.0));
        assert_eq!(Rational::new(3.0, 2.0).reduce(), Rational::new(3.0, 2.0));
        assert_eq!(
            Rational::new(12341241241.0, 123123312213132312132213.0).reduce(),
            Rational::new(12341241241.0, 123123312213132312132213.0)
        );
    }

    #[test]
    fn to_float() {
        assert_eq!(Rational::new(10.0, -2.0).to_float(), -5.0);
        assert_eq!(Rational::new(3.0, 2.0).to_float(), 3.0 / 2.0);
    }

    #[test]
    fn from_float() {
        assert_eq!(Rational::from(3.4), Rational::new(17.0, 5.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 1.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 2.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 3.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 4.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 5.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 6.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 7.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 8.0).reduce());
        assert_eq!(Rational::from(0.0), Rational::new(0.0, 9.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(1.0, 1.0).reduce());
        assert_eq!(Rational::from(0.5), Rational::new(1.0, 2.0).reduce());
        assert_eq!(
            Rational::from(0.3333333333333333),
            Rational::new(3333333333333333.0, 10000000000000000.0).reduce()
        );
        assert_eq!(Rational::from(0.25), Rational::new(1.0, 4.0).reduce());
        assert_eq!(Rational::from(0.2), Rational::new(1.0, 5.0).reduce());
        assert_eq!(Rational::from(0.125), Rational::new(1.0, 8.0).reduce());
        assert_eq!(Rational::from(2.0), Rational::new(2.0, 1.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(2.0, 2.0).reduce());
        assert_eq!(Rational::from(0.5), Rational::new(2.0, 4.0).reduce());
        assert_eq!(Rational::from(0.4), Rational::new(2.0, 5.0).reduce());
        assert_eq!(Rational::from(0.25), Rational::new(2.0, 8.0).reduce());
        assert_eq!(Rational::from(3.0), Rational::new(3.0, 1.0).reduce());
        assert_eq!(Rational::from(1.5), Rational::new(3.0, 2.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(3.0, 3.0).reduce());
        assert_eq!(Rational::from(0.75), Rational::new(3.0, 4.0).reduce());
        assert_eq!(Rational::from(0.6), Rational::new(3.0, 5.0).reduce());
        assert_eq!(Rational::from(0.5), Rational::new(3.0, 6.0).reduce());
        assert_eq!(Rational::from(0.375), Rational::new(3.0, 8.0).reduce());
        assert_eq!(Rational::from(4.0), Rational::new(4.0, 1.0).reduce());
        assert_eq!(Rational::from(2.0), Rational::new(4.0, 2.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(4.0, 4.0).reduce());
        assert_eq!(Rational::from(0.8), Rational::new(4.0, 5.0).reduce());
        assert_eq!(Rational::from(0.5), Rational::new(4.0, 8.0).reduce());
        assert_eq!(Rational::from(5.0), Rational::new(5.0, 1.0).reduce());
        assert_eq!(Rational::from(2.5), Rational::new(5.0, 2.0).reduce());
        assert_eq!(Rational::from(1.25), Rational::new(5.0, 4.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(5.0, 5.0).reduce());
        assert_eq!(Rational::from(0.625), Rational::new(5.0, 8.0).reduce());
        assert_eq!(Rational::from(6.0), Rational::new(6.0, 1.0).reduce());
        assert_eq!(Rational::from(3.0), Rational::new(6.0, 2.0).reduce());
        assert_eq!(Rational::from(2.0), Rational::new(6.0, 3.0).reduce());
        assert_eq!(Rational::from(1.5), Rational::new(6.0, 4.0).reduce());
        assert_eq!(Rational::from(1.2), Rational::new(6.0, 5.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(6.0, 6.0).reduce());
        assert_eq!(Rational::from(0.75), Rational::new(6.0, 8.0).reduce());
        assert_eq!(Rational::from(7.0), Rational::new(7.0, 1.0).reduce());
        assert_eq!(Rational::from(3.5), Rational::new(7.0, 2.0).reduce());
        assert_eq!(Rational::from(1.75), Rational::new(7.0, 4.0).reduce());
        assert_eq!(Rational::from(1.4), Rational::new(7.0, 5.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(7.0, 7.0).reduce());
        assert_eq!(Rational::from(0.875), Rational::new(7.0, 8.0).reduce());
        assert_eq!(Rational::from(8.0), Rational::new(8.0, 1.0).reduce());
        assert_eq!(Rational::from(4.0), Rational::new(8.0, 2.0).reduce());
        assert_eq!(Rational::from(2.0), Rational::new(8.0, 4.0).reduce());
        assert_eq!(Rational::from(1.6), Rational::new(8.0, 5.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(8.0, 8.0).reduce());
        assert_eq!(Rational::from(9.0), Rational::new(9.0, 1.0).reduce());
        assert_eq!(Rational::from(4.5), Rational::new(9.0, 2.0).reduce());
        assert_eq!(Rational::from(3.0), Rational::new(9.0, 3.0).reduce());
        assert_eq!(Rational::from(2.25), Rational::new(9.0, 4.0).reduce());
        assert_eq!(Rational::from(1.8), Rational::new(9.0, 5.0).reduce());
        assert_eq!(Rational::from(1.5), Rational::new(9.0, 6.0).reduce());
        assert_eq!(Rational::from(1.125), Rational::new(9.0, 8.0).reduce());
        assert_eq!(Rational::from(1.0), Rational::new(9.0, 9.0).reduce());
        // assert_eq!(Rational::new(3.0, 2.0).to_float(), 3.0 / 2.0);
    }
}
