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
    pub fn apply(&self) -> Rational {
        let out = match self {
            Op::Number(x) => *x,
            Op::Mul(x, y) => x.apply() * y.apply(),
            Op::Div(x, y) => x.apply() / y.apply(),
            Op::Add(x, y) => x.apply() + y.apply(),
            Op::Sub(x, y) => x.apply() - y.apply(),
            Op::Pow(x, y) => x.apply().pow(y.apply()),
            Op::Root(x) => x.apply().sqrt(),
            Op::Log(x, y) => y.apply().log(x.apply()),
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
