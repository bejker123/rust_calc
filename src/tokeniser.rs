use std::fmt::{Debug, Write};

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
    Root,
    Log,
}

impl Op {
    pub fn apply(&self, x: Option<f64>, y: Option<f64>, z: Option<f64>) -> Token {
        // let data = data.iter().flatten().cloned().collect::<Vec<f64>>();
        match self {
            Op::Mul => Token::Number(x.unwrap() * y.unwrap()),
            Op::Div => Token::Number(x.unwrap() / y.unwrap()),
            Op::Add => Token::Number(x.unwrap() + y.unwrap()),
            Op::Sub => Token::Number(x.unwrap() - y.unwrap()),
            Op::Pow => Token::Number(x.unwrap().powf(y.unwrap())),
            Op::Root => Token::Number(x.unwrap().sqrt()),
            Op::Log => Token::Number(x.unwrap().log(y.unwrap())),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Kilometer,
    Meter,
    Milimeter,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number,
    Op,
    Unit,
    Invalid,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Op(Op),
    Unit(Unit),
    Invalid,
}

impl Token {
    pub fn get_type(&self) -> TokenType {
        match self {
            Token::Number(_) => TokenType::Number,
            Token::Op(_) => TokenType::Op,
            Token::Unit(_) => TokenType::Unit,
            Token::Invalid => TokenType::Invalid,
        }
    }
    pub fn as_op(&self) -> Option<Op> {
        let Token::Op(x) = self else { return None };
        Some(x.clone())
    }
    pub fn as_nr(&self) -> Option<f64> {
        if let Token::Number(x) = self {
            return Some(x.clone());
        }
        None
    }
}

// impl Token {
//     pub fn is_valid(&self) -> bool {
//         if let a Token::Invalid = self {
//             return false;
//         }
//         true
//     }
// }

pub trait DbgDisplay {
    fn dbg(&self) -> Result<String, std::fmt::Error>;
}

impl DbgDisplay for Vec<Token> {
    fn dbg(&self) -> Result<String, std::fmt::Error> {
        let mut ret = String::new();
        for i in self {
            write!(ret, "{i:?}\n")?;
        }
        Ok(ret)
    }
}

impl DbgDisplay for Vec<(String, Token)> {
    fn dbg(&self) -> Result<String, std::fmt::Error> {
        let mut ret = String::new();
        for i in self {
            write!(ret, "{} {:?}\n", i.0, i.1)?;
        }
        Ok(ret)
    }
}

pub fn dbg_tokenise(s: &str) -> String {
    s.split(" ")
        .map(|x| (x.to_string(), _tokenise(x)))
        .collect::<Vec<(String, Token)>>()
        .dbg()
        .unwrap_or("Failed to tokenise".to_string())
}

pub fn tokenise(s: &str) -> Vec<Token> {
    s.split(" ").map(_tokenise).collect()
}

fn _tokenise(x: &str) -> Token {
    let x = x.to_lowercase();
    match x.as_str() {
        "*" => Token::Op(Op::Mul),
        "/" => Token::Op(Op::Div),
        "+" => Token::Op(Op::Add),
        "-" => Token::Op(Op::Sub),
        "^" => Token::Op(Op::Pow),
        "rt" | "root" => Token::Op(Op::Root),
        "log" | "lg" => Token::Op(Op::Log),

        y => {
            if let Ok(o) = y.parse::<f64>() {
                Token::Number(o)
            } else {
                Token::Invalid
            }
        }
    }
}
