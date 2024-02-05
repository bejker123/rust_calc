use crate::{DbgDisplay, Op, Token, TokenType};

pub trait Parse {
    fn parse(self) -> Result<(), String>;
}

impl Parse for Vec<Token> {
    fn parse(self) -> Result<(), String> {
        parse(self)
    }
}

fn sanitase(mut data: Vec<Token>) -> Result<Vec<Token>, String> {
    if data.contains(&Token::Invalid) {
        return Err(String::from("Stream contains invalid tokens"));
    }
    let mut prev_type = TokenType::Invalid;
    for (i, j) in data.clone().iter().enumerate() {
        let type_ = j.get_type();
        if type_ == TokenType::Number && prev_type == TokenType::Number {
            data.insert(i, Token::Op(Op::Mul));
        }
        prev_type = type_;
    }

    println!("After sanitase:\n{}", data.dbg().unwrap());
    Ok(data)
}

pub fn parse(data: Vec<Token>) -> Result<(), String> {
    let data = sanitase(data)?;
    let mut prev_token = Token::Invalid;
    let mut prev_type = TokenType::Invalid;
    let mut idx = 0;
    for i in data.iter() {
        if i.get_type() == TokenType::Op {
            let x = i.as_op().unwrap().apply(
                prev_token.as_nr(),
                data.get(idx + 1).unwrap_or(&Token::Invalid).clone().as_nr(),
                data.get(idx + 2).unwrap_or(&Token::Invalid).clone().as_nr(),
            );
            println!("After _parse: {x:?}");
        }
        prev_token = i.clone();
        prev_type = i.get_type();
        idx += 1;
    }

    Ok(())
}

// fn _parse(op: Op, data: Vec<Token>) -> Token {
//     op.apply(data.iter().map(|x| x.as_nr()).collect())
// }
