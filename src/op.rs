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

    // fn as_mul(&self) -> Self{
    //     match self{
    //         Op::Mul(_,_) => *self,
    //         Op::Div(x,y) => Op::Mul(*x,*y),
    //         Op::Add(x,y) => Op::Mul(*x,*y),
    //         Op::Sub(x,y) => Op::Mul(*x,*y),
    //         Op::Pow(x,y) => Op::Mul(*x,*y),
    //         _=> unreachable!()
    //     }
    // }
    //
    // fn as_div(&self) -> Self{
    //     match self{
    //         Op::Mul(x,y) => Op::Div(*x,*y),
    //         Op::Div(_,_) => *self,
    //         Op::Add(x,y) => Op::Div(*x,*y),
    //         Op::Sub(x,y) => Op::Div(*x,*y),
    //         Op::Pow(x,y) => Op::Div(*x,*y),
    //         _=> unreachable!()
    //     }
    // }
    //
    // fn as_add(&self) -> Self{
    //     match self{
    //         Op::Mul(x,y) => Op::Add(*x,*y),
    //         Op::Div(x,y) => Op::Add(*x,*y),
    //         Op::Add(_,_) => *self,
    //         Op::Sub(x,y) => Op::Add(*x,*y),
    //         Op::Pow(x,y) => Op::Add(*x,*y),
    //         _=> unreachable!()
    //     }
    // }
    //
    // fn as_(&self) -> Self{
    //     match self{
    //         Op::Mul(x,y) => Op::Add(*x,*y),
    //         Op::Div(x,y) => Op::Add(*x,*y),
    //         Op::Add(_,_) => *self,
    //         Op::Sub(x,y) => Op::Add(*x,*y),
    //         Op::Pow(x,y) => Op::Add(*x,*y),
    //         _=> unreachable!()
    //     }
    // }

    // pub fn swap(&mut self, other: &mut Op) -> &Op {
    //     match (self, other) {
    //         (Op::Number(x), Op::Number(y)) => {
    //             *self = Op::Number(*y);
    //             *other = Op::Number(*x)
    //         }
    //         (Op::Mul(x, y),)
    //     }
    //
    //     other
    // }

    pub fn get_order(&self) -> u8 {
        match self {
            Op::Pow(_, _) | Op::Root(_) | Op::Log(_, _) => 3,
            Op::Mul(_, _) | Op::Div(_, _) => 2,
            Op::Add(_, _) | Op::Sub(_, _) => 1,
            Op::Number(_) => 0,
        }
    }
}
