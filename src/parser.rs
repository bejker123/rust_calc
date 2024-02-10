use crate::{op::Op, OpType, Rational, Token, TokenType};

pub trait Parse {
    fn parse(self) -> Result<Rational, String>;
}

impl Parse for Vec<Token> {
    fn parse(self) -> Result<Rational, String> {
        Ok(parse_to_operations(sanitase(self)?)?.apply())
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
        let op_append = false;
        // if type_ == TokenType::Op {
        //     if j.as_op().unwrap().is_forward() {
        //         op_append = true;
        //     }
        // }
        if (type_ == TokenType::Number || op_append) && prev_type == TokenType::Number {
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
        Box::new(Op::Number(
            $data
                .get($idx + $i)
                .ok_or(format!("Expected token at: {}", $idx + $i))?
                .as_nr()
                .ok_or(String::from("Number expected."))?,
        ))
    };
}

macro_rules! prev_token {
    ($prev_token: expr) => {
        Box::new(Op::Number($prev_token.as_nr().ok_or("Number expected.")?))
    };
}

fn parse_to_operations(data: Vec<Token>) -> Result<Op, String> {
    // println!("parse_to_operations:");
    if data.contains(&Token::Invalid) {
        return Err(String::from("Stream contains invalid tokens"));
    }
    if data.len() >= 1 {
        return Ok(Op::Number(
            data.first()
                .ok_or("Stream empty")?
                .as_nr()
                .ok_or("Failed to parse stream.")?,
        ));
    }
    let mut prev_token = Token::Invalid;
    let mut skip = 0;
    let mut ret = Op::Number(Rational::zero());
    for i in 0..data.len() {
        let token = data.get(i).unwrap();
        if skip > 0 {
            skip -= 1;
            prev_token = token.clone();
            continue;
        }

        // println!("{idx}: {i:?}");
        if token.get_type() == TokenType::Op {
            let op = token.as_op().unwrap();
            let reverse = ret.get_order() < op.get_order() && ret.get_order() != 0;
            match op {
                OpType::Mul => {
                    skip += 1;
                    let mut prev = if ret != Op::Number(Rational::zero()) {
                        Box::new(ret.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    /*
                    >1+2/3
                    Add(Number(1), Number(2))
                    Div(Add(Number(1), Number(2)), Number(3))
                    =1
                    >Add(Number(1),Div(Number(2),Number(3))
                    */

                    if reverse {
                        if let Some(y) = prev.get_y() {
                            prev.change_y(Box::new(Op::Mul(y, next_token!(data, i, 1))));
                            ret = *prev.clone();
                        }
                    } else {
                        ret = Op::Mul(prev, next_token!(data, i, 1));
                    }
                }
                OpType::Div => {
                    skip += 1;
                    let mut prev = if ret != Op::Number(Rational::zero()) {
                        Box::new(ret.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    if reverse {
                        if let Some(y) = prev.get_y() {
                            prev.change_y(Box::new(Op::Div(y, next_token!(data, i, 1))));
                            ret = *prev.clone();
                        }
                    } else {
                        ret = Op::Div(prev, next_token!(data, i, 1));
                    }
                }
                OpType::Add => {
                    skip += 1;
                    let mut prev = if ret != Op::Number(Rational::zero()) {
                        Box::new(ret.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    if reverse {
                        if let Some(y) = prev.get_y() {
                            prev.change_y(Box::new(Op::Add(y, next_token!(data, i, 1))));
                            ret = *prev.clone();
                        }
                    } else {
                        ret = Op::Add(prev, next_token!(data, i, 1));
                    }
                }
                OpType::Sub => {
                    skip += 1;
                    let mut prev = if ret != Op::Number(Rational::zero()) {
                        Box::new(ret.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    if reverse {
                        if let Some(y) = prev.get_y() {
                            prev.change_y(Box::new(Op::Sub(y, next_token!(data, i, 1))));
                            ret = *prev.clone();
                        }
                    } else {
                        ret = Op::Sub(prev, next_token!(data, i, 1));
                    }
                }
                OpType::Pow => {
                    skip += 1;
                    let mut prev = if ret != Op::Number(Rational::zero()) {
                        Box::new(ret.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    if reverse {
                        if let Some(y) = prev.get_y() {
                            prev.change_y(Box::new(Op::Pow(y, next_token!(data, i, 1))));
                            ret = *prev.clone();
                        }
                    } else {
                        ret = Op::Pow(prev, next_token!(data, i, 1));
                    }
                }
                OpType::Log => {
                    if reverse {
                        todo!();
                    }
                    skip += 2;
                    ret = Op::Log(next_token!(data, i, 1), next_token!(data, i, 2));
                }
                OpType::Root => {
                    if reverse {
                        todo!();
                    }
                    skip += 1;
                    ret = Op::Root(next_token!(data, i, 1));
                }
            }
        }
        prev_token = token.clone();
    }
    Ok(ret)
}

mod test {
    #[cfg(test)]
    use super::{parse_to_operations, Op, OpType, Token};

    #[test]
    fn test_parse_to_operations() {
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Mul),
                Token::Number(2.0.into()),
            ])
            .unwrap(),
            Op::Mul(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Div),
                Token::Number(2.0.into()),
            ])
            .unwrap(),
            Op::Div(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Add),
                Token::Number(2.0.into()),
            ])
            .unwrap(),
            Op::Add(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Sub),
                Token::Number(2.0.into()),
            ])
            .unwrap(),
            Op::Sub(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        // assert_eq!(
        //     parse_to_operations(vec![Token::Op(OpType::Root), Token::Number(2.0.into()),]),
        //     vec![Op::Root(Box::new(Op::Number(2.0.into())))]
        // );
    }
}
