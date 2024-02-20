use crate::Rational;

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
    Root,
    Log,
}

impl OpType {
    pub fn is_forward(&self) -> bool {
        match self {
            OpType::Mul => false,
            OpType::Div => false,
            OpType::Add => false,
            OpType::Sub => false,
            OpType::Pow => false,
            OpType::Root => true,
            OpType::Log => true,
        }
    }
    pub fn get_order(&self) -> u8 {
        match self {
            OpType::Pow | OpType::Root | OpType::Log => 3,
            OpType::Mul | OpType::Div => 2,
            OpType::Add | OpType::Sub => 1,
        }
    }
    pub fn get_consume_count(&self) -> u8 {
        match self {
            OpType::Mul => 2,
            OpType::Div => 2,
            OpType::Add => 2,
            OpType::Sub => 2,
            OpType::Pow => 2,
            OpType::Root => 1,
            OpType::Log => 2,
        }
    }
}
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
    pub fn from_type(t: OpType, x: Option<Box<Op>>, y: Option<Box<Op>>) -> Self {
        let x = x.unwrap_or(Box::new(Op::Number(Rational::zero())));
        let y = y.unwrap_or(Box::new(Op::Number(Rational::zero())));
        match t {
            OpType::Mul => Op::Mul(x, y),
            OpType::Div => Op::Div(x, y),
            OpType::Add => Op::Add(x, y),
            OpType::Sub => Op::Sub(x, y),
            OpType::Pow => Op::Pow(x, y),
            OpType::Root => Op::Root(x),
            OpType::Log => Op::Log(x, y),
        }
    }

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

    pub fn get_type(&self) -> Option<OpType> {
        match self {
            Op::Mul(_, _) => Some(OpType::Mul),
            Op::Div(_, _) => Some(OpType::Div),
            Op::Add(_, _) => Some(OpType::Add),
            Op::Sub(_, _) => Some(OpType::Sub),
            Op::Pow(_, _) => Some(OpType::Pow),
            Op::Root(_) => Some(OpType::Root),
            Op::Log(_, _) => Some(OpType::Log),
            Op::Number(_) => None,
        }
    }

    pub fn get_order(&self) -> u8 {
        if let Some(o) = self.get_type() {
            o.get_order()
        } else {
            0
        }
    }
}
