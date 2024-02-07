#[derive(Clone, Copy, PartialEq, Debug)]
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
    }

    pub fn sqrt(&self) -> Self {
        Self {
            p: self.p.sqrt(),
            q: self.q.sqrt(),
        }
    }
}

impl std::fmt::Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("{}", self.to_float()))
        let (p, q) = self.reduce().into();
        f.write_fmt(format_args!("{}/{}", p, q))
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
}