use std::collections::HashMap;

use crate::op::{Op, OpType};
use crate::{rational::Rational, tokenizer::DbgDisplay, tokenizer::Token, tokenizer::TokenType};

pub type KnownLiterals = HashMap<String, Rational>;

pub trait Parse {
    fn parse(self, known_literals: &mut KnownLiterals) -> Result<Rational, String>;
}

impl Parse for Vec<Token> {
    fn parse(self, known_literals: &mut KnownLiterals) -> Result<Rational, String> {
        Ok(parse_to_operations(sanitase(self)?, known_literals)?.apply())
    }
}

impl Parse for Vec<(String, Token)> {
    fn parse(self, known_literals: &mut KnownLiterals) -> Result<Rational, String> {
        println!(
            "{}",
            self.dbg()
                .unwrap_or(String::from("Failed to display token stream"))
        );
        let out = self.into_iter().map(|x| x.1).collect();
        Ok(parse_to_operations(sanitase(out)?, known_literals)?.apply())
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
        if (type_ == TokenType::Number || type_ == TokenType::Literal || op_append)
            && (prev_type == TokenType::Number || prev_type == TokenType::Literal)
        {
            if i >= 2 {
                if !(data[i - 2].get_type() == TokenType::Op
                    && data[i - 2].as_op_type().unwrap().is_forward())
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

fn parse_to_operations(data: Vec<Token>, known_literals: &mut KnownLiterals) -> Result<Op, String> {
    // println!("parse_to_operations:");
    if data.contains(&Token::Invalid) {
        return Err(String::from("Stream contains invalid tokens"));
    }

    let data = data
        .iter()
        .map(|token| {
            //Proactive literal lookup
            if let Token::Literal(lit) = token.clone() {
                if let Some(val) = known_literals.get(&lit) {
                    return Token::Number(*val);
                }
            }
            return token.clone();
        })
        .collect::<Vec<_>>();

    if data.len() <= 1 {
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

        if let Some(op_type) = token.as_op_type() {
            let reverse = ret.get_order() < op_type.get_order() && ret.get_order() != 0;

            skip += op_type.get_consume_count();
            if op_type.is_forward() {
                if reverse {
                    todo!();
                }
                if op_type == OpType::Root {
                    ret = Op::from_type(op_type, Some(next_token!(data, i, 1)), None);
                } else {
                    ret = Op::from_type(
                        op_type,
                        Some(next_token!(data, i, 1)),
                        Some(next_token!(data, i, 2)),
                    );
                }
            } else {
                skip -= 1;
                let mut prev = if ret != Op::Number(Rational::zero()) {
                    Box::new(ret.clone())
                } else {
                    prev_token!(prev_token)
                };

                if reverse {
                    if let Some(y) = prev.get_y() {
                        prev.change_y(Box::new(Op::from_type(
                            op_type,
                            Some(y),
                            Some(next_token!(data, i, 1)),
                        )));
                        ret = *prev.clone();
                    }
                } else {
                    ret = Op::from_type(op_type, Some(prev), Some(next_token!(data, i, 1)));
                }
            }
        } else if token.get_type() == TokenType::Eq {
            if let Token::Literal(prev) = prev_token.clone() {
                let next_token = data
                    .get(i + 1)
                    .ok_or(format!("Expected token at: {}", i + 1))?;
                if let Token::Literal(next) = next_token {
                    if let Some(next) = known_literals.get(next) {
                        ret = Op::Number(next.clone());
                        known_literals.insert(prev, *next);
                    }
                } else if let Token::Number(next) = next_token {
                    ret = Op::Number(next.clone());
                    known_literals.insert(prev, *next);
                }
            }
        }
        //Retroactive literal lookup
        if let Token::Literal(lit) = token.clone() {
            if let Some(val) = known_literals.get(&lit) {
                prev_token = Token::Number(*val);
                ret = Op::Number(*val);
            } else {
                if i + 1 > data.len() - 1 {
                    return Err(format!("Unknown literal: {lit:?}"));
                }
                prev_token = token.clone();
            }
        } else {
            prev_token = token.clone();
        }
    }
    Ok(ret)
}

mod test {

    #[cfg(test)]
    use super::{parse_to_operations, KnownLiterals, Op, OpType, Token};

    #[test]
    fn test_parse_to_operations() {
        let mut known_literals = KnownLiterals::new();
        assert_eq!(
            parse_to_operations(
                vec![
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Mul),
                    Token::Number(2.0.into()),
                ],
                &mut known_literals
            )
            .unwrap(),
            Op::Mul(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(
                vec![
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Div),
                    Token::Number(2.0.into()),
                ],
                &mut known_literals
            )
            .unwrap(),
            Op::Div(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(
                vec![
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Add),
                    Token::Number(2.0.into()),
                ],
                &mut known_literals
            )
            .unwrap(),
            Op::Add(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(
                vec![
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Sub),
                    Token::Number(2.0.into()),
                ],
                &mut known_literals
            )
            .unwrap(),
            Op::Sub(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Number(2.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(
                vec![
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Sub),
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Div),
                    Token::Number(3.0.into()),
                ],
                &mut known_literals
            )
            .unwrap(),
            Op::Sub(
                Box::new(Op::Number(2.0.into())),
                Box::new(Op::Div(
                    Box::new(Op::Number(2.0.into())),
                    Box::new(Op::Number(3.0.into()))
                ))
            )
        );
        assert_eq!(
            parse_to_operations(
                vec![
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Mul),
                    Token::Number(2.0.into()),
                    Token::Op(OpType::Div),
                    Token::Number(3.0.into()),
                ],
                &mut known_literals
            )
            .unwrap(),
            Op::Div(
                Box::new(Op::Mul(
                    Box::new(Op::Number(2.0.into())),
                    Box::new(Op::Number(2.0.into()))
                )),
                Box::new(Op::Number(3.0.into()))
            )
        );
        assert_eq!(
            parse_to_operations(
                vec![Token::Op(OpType::Root), Token::Number(2.0.into())],
                &mut known_literals
            )
            .unwrap(),
            Op::Root(Box::new(Op::Number(2.0.into())))
        );
    }
}
