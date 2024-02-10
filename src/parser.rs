use crate::{op::Op, OpType, Rational, Token, TokenType};

pub trait Parse {
    fn parse(self) -> Result<Rational, String>;
}

impl Parse for Vec<Token> {
    fn parse(self) -> Result<Rational, String> {
        Ok(parse(parse_to_operations(order(sanitase(self)?))?))
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
        let mut op_append = false;
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

fn order(mut data: Vec<Token>) -> Vec<Token> {
    if data.len() <= 1 {
        return data;
    }

    // println!("Before order:\n\t{:?}", data);
    // let mut last_op: Option<OpType> = None;
    // let mut last_op_idx: Option<usize> = None;
    // let mut idx = 0;
    // for i in data.clone().into_iter() {
    //     match i {
    //         Token::Op(curr_op) => {
    //             match last_op {
    //                 Some(last_op) => {
    //                     if curr_op.get_order() > last_op.get_order() {
    //                         data.swap(idx, last_op_idx.unwrap());
    //                         data.swap(idx - 1, idx - 3);
    //                         data.swap(idx - 1, idx + 1);
    //                     }
    //                 }
    //                 None => {}
    //             };
    //             last_op = Some(curr_op.clone());
    //             last_op_idx = Some(idx);
    //         }
    //         _ => {}
    //     }
    //     idx += 1;
    // }
    // println!("after order:\n\t{:?}", data);
    return data;
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
        Box::new(Op::Number(
            $prev_token
                .as_nr()
                .ok_or(String::from("Number expected."))?,
        ))
    };
}

fn parse_to_operations(data: Vec<Token>) -> Result<Vec<Op>, String> {
    // println!("parse_to_operations:");
    if data.contains(&Token::Invalid) {
        return Err(String::from("Stream contains invalid tokens"));
    }
    if data.len() == 1 {
        return Ok(vec![Op::Number(data.first().unwrap().as_nr().unwrap())]);
    }
    let mut ret = Vec::new();
    let mut prev_token = Token::Invalid;
    let mut skip = 0;
    let mut prev_op = Op::Number(Rational::zero());
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
            if prev_op.get_order() < op.get_order() {}
            match op {
                OpType::Mul => {
                    skip += 1;
                    let prev = if !ret.is_empty() {
                        Box::new(prev_op.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    prev_op = Op::Mul(prev, next_token!(data, i, 1));
                }
                OpType::Div => {
                    let prev = if !ret.is_empty() {
                        Box::new(prev_op.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    skip += 1;
                    prev_op = Op::Div(prev, next_token!(data, i, 1));
                }
                OpType::Add => {
                    let prev = if !ret.is_empty() {
                        Box::new(prev_op.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    skip += 1;
                    prev_op = Op::Add(prev, next_token!(data, i, 1));
                }
                OpType::Sub => {
                    let prev = if !ret.is_empty() {
                        Box::new(prev_op.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    skip += 1;
                    prev_op = Op::Sub(prev, next_token!(data, i, 1));
                }
                OpType::Pow => {
                    let prev = if !ret.is_empty() {
                        Box::new(prev_op.clone())
                    } else {
                        prev_token!(prev_token)
                    };
                    skip += 1;
                    prev_op = Op::Pow(prev, next_token!(data, i, 1));
                }
                OpType::Log => {
                    skip += 2;
                    prev_op = Op::Log(next_token!(data, i, 1), next_token!(data, i, 2));
                }
                OpType::Root => {
                    skip += 1;
                    prev_op = Op::Root(next_token!(data, i, 1));
                }
                _ => {}
            }
            ret.push(prev_op.clone());
        }
        prev_token = token.clone();
    }
    Ok(ret)
}

pub fn parse(data: Vec<Op>) -> Rational {
    let mut ret = Rational::zero();
    if data.is_empty() {
        return ret;
    }
    // let data = sanitase(data)?;
    // let mut idx = 0;
    for i in data.iter() {
        println!("{i:?}");
        match i.apply() {
            Ok(_) => {
                unreachable!()
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
    #[cfg(test)]
    use super::{parse_to_operations, Op, OpType, Token};

    #[test]
    fn test_parse_to_operations() {
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Mul),
                Token::Number(2.0.into()),
            ]),
            vec![Op::Mul(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )]
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Div),
                Token::Number(2.0.into()),
            ]),
            vec![Op::Div(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )]
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Add),
                Token::Number(2.0.into()),
            ]),
            vec![Op::Add(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )]
        );
        assert_eq!(
            parse_to_operations(vec![
                Token::Number(2.0.into()),
                Token::Op(OpType::Sub),
                Token::Number(2.0.into()),
            ]),
            vec![Op::Sub(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )]
        );
        // assert_eq!(
        //     parse_to_operations(vec![Token::Op(OpType::Root), Token::Number(2.0.into()),]),
        //     vec![Op::Root(Box::new(Op::Number(2.0.into())))]
        // );
    }
}
