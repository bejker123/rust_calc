use crate::op::OpType;
use std::fmt::{Debug, Write};

use crate::rational::Rational;

#[derive(Clone, Copy, Debug, Default)]
pub struct TokenizerOptions {
    pub debug: bool,
    pub as_float: bool,
}

// #[derive(Debug, PartialEq, Clone)]
// pub enum Unit {
//     Kilometer,
//     Meter,
//     Milimeter,
// }

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number,
    Op,
    // Unit,
    OpenP,
    CloseP,
    Literal,
    Eq,
    Invalid,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(Rational),
    Op(OpType),
    // Unit(Unit),
    OpenP,
    CloseP,
    Literal(String),
    Eq,
    Invalid,
}

impl Token {
    pub fn get_type(&self) -> TokenType {
        match self {
            Token::Number(_) => TokenType::Number,
            Token::Op(_) => TokenType::Op,
            // Token::Unit(_) => TokenType::Unit,
            Token::OpenP => TokenType::OpenP,
            Token::CloseP => TokenType::CloseP,
            Token::Literal(_) => TokenType::Literal,
            Token::Eq => TokenType::Eq,
            Token::Invalid => TokenType::Invalid,
        }
    }
    pub fn as_op_type(&self) -> Option<OpType> {
        let Token::Op(x) = self else { return None };
        Some(x.clone())
    }
    pub fn as_nr(&self) -> Option<Rational> {
        if let Token::Number(x) = self {
            return Some(x.clone());
        }
        None
    }
}

pub trait DbgDisplay {
    fn dbg(&self) -> Result<String, std::fmt::Error>;
}

impl DbgDisplay for Vec<Token> {
    fn dbg(&self) -> Result<String, std::fmt::Error> {
        let mut ret = String::new();
        for i in 0..self.len() {
            write!(ret, "{:?}\r", self[i])?;
            if i < self.len() - 1 {
                write!(ret, "\n")?;
            }
        }
        Ok(ret)
    }
}

impl DbgDisplay for Vec<(String, Token)> {
    fn dbg(&self) -> Result<String, std::fmt::Error> {
        let mut ret = String::new();
        for i in 0..self.len() {
            write!(ret, "{} {:?}\r", self[i].0, self[i].1)?;
            if i < self.len() - 1 {
                write!(ret, "\n")?;
            }
        }
        Ok(ret)
    }
}

fn split<'a>(mut s: &'a str) -> Vec<&'a str> {
    let pats = [' ', '*', '/', '+', '-', '^', '(', ')', '%', '='];
    let mut ret = Vec::new();
    // println!("Splitting: {s:?}");
    loop {
        let mut idxs = Vec::new();
        for i in pats {
            // println!("Looking for: {i:?} in {s:?}");
            if let Some(x) = s.find(i) {
                idxs.push(x);
            }
        }
        // println!("Found: {idxs:?}");
        if idxs.is_empty() {
            // println!("Left: {s:?}");
            if !s.is_empty() {
                ret.push(s);
            }
            break;
        } else {
            idxs.sort();
            let x = idxs.first().unwrap().to_owned();
            // println!("Found: {x:?} {ln:?}");
            let to_push = &s[..x];
            if !to_push.is_empty() {
                ret.push(to_push);
                // println!("{to_push:?} {s:?}");
            }
            let delim = &s[x..x + 1];
            // println!("delim: {delim:?}");
            if delim != " " {
                ret.push(delim);
            }
            s = &s[x + 1..];
        }
    }
    ret
}

pub fn pre_tokenize(s: &str) -> (&str, TokenizerOptions) {
    let mut ret = TokenizerOptions::default();
    if let Some(sep_idx) = s.find('#') {
        for i in s[..sep_idx].chars().into_iter() {
            match i {
                'd' => {
                    ret.debug = true;
                }
                'f' => {
                    ret.as_float = true;
                }
                _ => {}
            }
        }
        (&s[sep_idx + 1..], ret)
    } else {
        (s, ret)
    }
}

pub fn dbg_tokenize(s: &str) -> Vec<(String, Token)> {
    split(s)
        .iter()
        .map(|x| (x.to_string(), _tokenize(x)))
        .collect()
    // .dbg()
    // .unwrap_or("Failed to tokenize".to_string())
}

pub fn tokenize(s: &str) -> Vec<Token> {
    split(s).into_iter().map(_tokenize).collect()
}

fn _tokenize(x: &str) -> Token {
    let x = x.to_lowercase();
    // println!("x: {x:?}");
    match x.as_str() {
        "*" => Token::Op(OpType::Mul),
        "/" => Token::Op(OpType::Div),
        "+" => Token::Op(OpType::Add),
        "-" => Token::Op(OpType::Sub),
        "^" => Token::Op(OpType::Pow),
        "sqrt" | "rt" | "root" => Token::Op(OpType::Root),
        "log" | "lg" => Token::Op(OpType::Log),
        "(" => Token::OpenP,
        ")" => Token::CloseP,
        "%" => Token::Op(OpType::Mod),
        "=" => Token::Eq,

        y => {
            if let Ok(o) = y.parse::<f64>() {
                Token::Number(Rational::from(o))
            } else {
                Token::Literal(y.to_owned())
            }
        }
    }
}

mod test {
    #[cfg(test)]
    use crate::tokenizer::*;

    #[test]
    fn test_split() {
        assert_eq!(split("a b"), vec!["a", "b"]);
        assert_eq!(split("a/b"), vec!["a", "/", "b"]);
        assert_eq!(split("a*b"), vec!["a", "*", "b"]);
        assert_eq!(split("a/ b"), vec!["a", "/", "b"]);
        assert_eq!(
            split("a/ b*/////"),
            vec!["a", "/", "b", "*", "/", "/", "/", "/", "/"]
        );
        assert_eq!(split("log a b"), vec!["log", "a", "b"]);
        assert_eq!(split("sqrt a ^ b"), vec!["sqrt", "a", "^", "b"]);
    }

    #[test]
    fn test_priv_tokenize() {
        assert_eq!(_tokenize("*"), Token::Op(OpType::Mul));
        assert_eq!(_tokenize("/"), Token::Op(OpType::Div));
        assert_eq!(_tokenize("+"), Token::Op(OpType::Add));
        assert_eq!(_tokenize("-"), Token::Op(OpType::Sub));
        assert_eq!(_tokenize("^"), Token::Op(OpType::Pow));
        assert_eq!(_tokenize("sqrt"), Token::Op(OpType::Root));
        assert_eq!(_tokenize("log"), Token::Op(OpType::Log));
        assert_eq!(_tokenize("123"), Token::Number(123.0.into()));
        assert_eq!(_tokenize("123.0"), Token::Number(123.0.into()));
        assert_eq!(_tokenize(".01"), Token::Number(Rational::new(1.0, 100.0)));
    }

    #[test]
    fn test_tokenize() {
        assert_eq!(
            tokenize("1/123"),
            vec![
                Token::Number(1.0.into()),
                Token::Op(OpType::Div),
                Token::Number(123.0.into())
            ]
        );
        assert_eq!(
            tokenize("1 123"),
            vec![Token::Number(1.0.into()), Token::Number(123.0.into())]
        );
        assert_eq!(
            tokenize("1*123"),
            vec![
                Token::Number(1.0.into()),
                Token::Op(OpType::Mul),
                Token::Number(123.0.into())
            ]
        );
        assert_eq!(
            tokenize("1*123/321"),
            vec![
                Token::Number(1.0.into()),
                Token::Op(OpType::Mul),
                Token::Number(123.0.into()),
                Token::Op(OpType::Div),
                Token::Number(321.0.into())
            ]
        );
        assert_eq!(
            tokenize("a 1*123"),
            vec![
                Token::Literal(String::from("a")),
                Token::Number(1.0.into()),
                Token::Op(OpType::Mul),
                Token::Number(123.0.into())
            ]
        );
    }
}
