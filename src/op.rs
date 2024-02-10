use crate::Rational;

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Mul(Box<Op>, Box<Op>),
    Div(Box<Op>, Box<Op>),
    Add(Box<Op>, Box<Op>),
    Sub(Box<Op>, Box<Op>),
    Pow(Box<Op>, Box<Op>),
    Root(Box<Op>),
    Log(Box<Op>, Box<Op>),
    Number(Rational),
}

impl Op {
    pub fn apply(&self) -> Result<Op, Rational> {
        let out = match self {
            Op::Number(x) => Err(x.clone()),
            Op::Mul(x, y) => match (*x.clone(), *y.clone()) {
                (x, y) => Err(x.apply().expect_err("") * y.apply().expect_err("")),
            },
            Op::Div(x, y) => match (*x.clone(), *y.clone()) {
                (x, y) => Err(x.apply().expect_err("") / y.apply().expect_err("")),
            },
            Op::Add(x, y) => match (*x.clone(), *y.clone()) {
                (x, y) => Err(x.apply().expect_err("") + y.apply().expect_err("")),
            },
            Op::Sub(x, y) => match (*x.clone(), *y.clone()) {
                (x, y) => Err(x.apply().expect_err("") - y.apply().expect_err("")),
            },
            Op::Pow(x, y) => match (*x.clone(), *y.clone()) {
                (x, y) => Err(x.apply().expect_err("").pow(y.apply().expect_err(""))),
            },
            Op::Root(x) => match *x.clone() {
                x => Err(x.apply().expect_err("").sqrt()),
            },
            Op::Log(x, y) => match (*x.clone(), *y.clone()) {
                (x, y) => Err(y.apply().expect_err("").log(x.apply().expect_err(""))),
            },
        };
        // println!("apply: self: {self:?} out: {out:?}");
        out
    }

    pub fn get_y(&self) -> Option<Box<Op>> {
        match self.clone() {
            Op::Pow(_, y)
            | Op::Log(_, y)
            | Op::Mul(_, y)
            | Op::Div(_, y)
            | Op::Add(_, y)
            | Op::Sub(_, y) => Some(y),
            _ => None,
        }
    }
    pub fn change_y(&mut self, new_y: Box<Op>) {
        match self {
            Op::Pow(_, ref mut y)
            | Op::Log(_, ref mut y)
            | Op::Mul(_, ref mut y)
            | Op::Div(_, ref mut y)
            | Op::Add(_, ref mut y)
            | Op::Sub(_, ref mut y) => {
                *y = new_y;
            }
            _ => {}
        }
    }

    pub fn get_order(&self) -> u8 {
        match self {
            Op::Pow(_, _) | Op::Root(_) | Op::Log(_, _) => 3,
            Op::Mul(_, _) | Op::Div(_, _) => 2,
            Op::Add(_, _) | Op::Sub(_, _) => 1,
            Op::Number(_) => 0,
        }
    }
}
