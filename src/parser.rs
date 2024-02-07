use crate::{DbgDisplay, OpType, Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Mul(Box<Op>, Box<Op>),
    Div(Box<Op>, Box<Op>),
    Add(Box<Op>, Box<Op>),
    Sub(Box<Op>, Box<Op>),
    Pow(Box<Op>, Box<Op>),
    Root(Box<Op>),
    Log(Box<Op>, Box<Op>),
    Number(f64),
}

impl Op {
    pub fn apply(&self) -> Result<Op, f64> {
        println!("{self:?}");
        match self {
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
                (x, y) => Err(x.apply().expect_err("").powf(y.apply().expect_err(""))),
            },
            Op::Root(x) => match *x.clone() {
                x => Err(x.apply().expect_err("").sqrt()),
            },
            Op::Log(x, y) => match (*x.clone(), *y.clone()) {
                (x, y) => Err(y.apply().expect_err("").log(x.apply().expect_err(""))),
            },
        }
    }
    pub fn from(tk: Token, op1: Option<Op>, op2: Option<Op>) -> Op {
        match tk {
            Token::Number(x) => Op::Number(x),
            Token::Op(OpType::Mul) => Op::Mul(Box::new(op1.unwrap()), Box::new(op2.unwrap())),
            Token::Op(OpType::Div) => Op::Div(Box::new(op1.unwrap()), Box::new(op2.unwrap())),
            Token::Op(OpType::Add) => Op::Add(Box::new(op1.unwrap()), Box::new(op2.unwrap())),
            Token::Op(OpType::Sub) => Op::Sub(Box::new(op1.unwrap()), Box::new(op2.unwrap())),
            Token::Op(OpType::Pow) => Op::Pow(Box::new(op1.unwrap()), Box::new(op2.unwrap())),
            Token::Op(OpType::Log) => Op::Log(Box::new(op1.unwrap()), Box::new(op2.unwrap())),
            Token::Op(OpType::Root) => Op::Root(Box::new(op1.unwrap())),
            _ => unreachable!(),
        }
    }
}

pub trait Parse {
    fn parse(self) -> Result<f64, String>;
}

impl Parse for Vec<Token> {
    fn parse(self) -> Result<f64, String> {
        Ok(parse(parse_to_operations(sanitase(self)?)))
    }
}

fn sanitase(mut data: Vec<Token>) -> Result<Vec<Token>, String> {
    if data.len() == 1 || data.is_empty() {
        return Ok(data);
    }
    if data.contains(&Token::Invalid) {
        return Err(String::from("Stream contains invalid tokens"));
    }
    let mut prev_type = TokenType::Invalid;
    for (i, j) in data.clone().iter().enumerate() {
        let type_ = j.get_type();
        if type_ == TokenType::Number && prev_type == TokenType::Number {
            if i >= 2 {
                if !(data[i - 2].get_type() == TokenType::Op
                    && data[i - 2].as_op().unwrap().is_forward())
                {
                    data.insert(i, Token::Op(OpType::Mul));
                }
            } else {
                data.insert(i, Token::Op(OpType::Mul));
            }
        }
        prev_type = type_;
    }

    // println!("After sanitase:\n{}", data.dbg().unwrap());
    Ok(data)
}

macro_rules! next_token {
    ($data: expr,$idx: expr,$i:expr) => {
        Box::new(Op::Number($data.get($idx + $i).unwrap().as_nr().unwrap()))
    };
}

macro_rules! prev_token {
    ($prev_token: expr) => {
        Box::new(Op::Number($prev_token.as_nr().unwrap()))
    };
}

fn parse_to_operations(data: Vec<Token>) -> Vec<Op> {
    if data.len() == 1 {
        return vec![Op::Number(data.first().unwrap().as_nr().unwrap())];
    }
    let mut ret = Vec::new();
    let mut prev_token = Token::Invalid;
    let mut idx = 0;
    let mut skip = 0;
    let mut prev_op = Op::Number(0.0);
    for i in data.iter() {
        if skip > 0 {
            skip -= 1;
            prev_token = i.clone();
            idx += 1;
            continue;
        }
        // println!("{idx}: {i:?}");
        if i.get_type() == TokenType::Op {
            let op = i.as_op().unwrap();
            let prev = if !ret.is_empty() {
                Box::new(prev_op.clone())
            } else {
                prev_token!(prev_token)
            };
            match op {
                OpType::Mul => {
                    skip += 1;
                    prev_op = Op::Mul(prev, next_token!(data, idx, 1));
                }
                OpType::Div => {
                    skip += 1;
                    prev_op = Op::Div(prev, next_token!(data, idx, 1));
                }
                OpType::Add => {
                    skip += 1;
                    prev_op = Op::Add(prev, next_token!(data, idx, 1));
                }
                OpType::Sub => {
                    skip += 1;
                    prev_op = Op::Sub(prev, next_token!(data, idx, 1));
                }
                OpType::Pow => {
                    skip += 1;
                    prev_op = Op::Pow(prev, next_token!(data, idx, 1));
                }
                OpType::Log => {
                    todo!();
                    skip += 2;
                    ret.push(Op::Log(
                        next_token!(data, idx, 1),
                        next_token!(data, idx, 2),
                    ))
                }
                OpType::Root => {
                    todo!();
                    skip += 1;
                    ret.push(Op::Root(next_token!(data, idx, 1)))
                }
                _ => todo!(),
            }
            ret.push(prev_op.clone());
        }
        prev_token = i.clone();
        idx += 1;
    }
    ret
}

pub fn parse(data: Vec<Op>) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    // let data = sanitase(data)?;
    // let mut idx = 0;
    let mut ret = 0.0;
    for i in data.iter() {
        match i.apply() {
            Ok(op) => {
                op.apply();
            }
            Err(x) => ret = x,
        }
    }

    // ret.extend_from_slice(&data[idx..]);
    // if ret.len() != 1 {
    //     return parse(ret);
    // }

    // Ok(ret)
    // Ok(vec![])
    ret
}

mod test {
    use super::parse_to_operations;
    use super::{Op, OpType, Token};

    #[test]
    fn test_parse_to_operations() {
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0),
                Token::Op(OpType::Mul),
                Token::Number(2.0),
            ]),
            vec![Op::Mul(
                Box::new(Op::Number(2.0)),
                Box::new(Op::Number(2.0))
            )]
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0),
                Token::Op(OpType::Div),
                Token::Number(2.0),
            ]),
            vec![Op::Div(
                Box::new(Op::Number(2.0)),
                Box::new(Op::Number(2.0))
            )]
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0),
                Token::Op(OpType::Add),
                Token::Number(2.0),
            ]),
            vec![Op::Add(
                Box::new(Op::Number(2.0)),
                Box::new(Op::Number(2.0))
            )]
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0),
                Token::Op(OpType::Sub),
                Token::Number(2.0),
            ]),
            vec![Op::Sub(
                Box::new(Op::Number(2.0)),
                Box::new(Op::Number(2.0))
            )]
        );
        // assert_eq!(
        //     parse_to_operations(vec![Token::Op(OpType::Root), Token::Number(2.0),]),
        //     vec![Op::Root(Box::new(Op::Number(2.0)))]
        // );
    }
}
