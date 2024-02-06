use crate::{DbgDisplay, Op, Token, TokenType};

pub trait Parse {
    fn parse(self) -> Result<Vec<Token>, String>;
}

impl Parse for Vec<Token> {
    fn parse(self) -> Result<Vec<Token>, String> {
        parse(self)
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
                    data.insert(i, Token::Op(Op::Mul));
                }
            } else {
                data.insert(i, Token::Op(Op::Mul));
            }
        }
        prev_type = type_;
    }

    // println!("After sanitase:\n{}", data.dbg().unwrap());
    Ok(data)
}

fn order(mut data: Vec<Token>) -> Vec<Token> {
    todo!()
}

pub fn parse(data: Vec<Token>) -> Result<Vec<Token>, String> {
    if data.len() == 1 {
        return Ok(data);
    }
    let data = sanitase(data)?;
    let mut prev_token = Token::Invalid;
    let mut idx = 0;
    let mut ret = Vec::new();
    for i in data.iter() {
        if i.get_type() == TokenType::Op {
            // println!("_parse data.len(): {}", data.len());
            let x = i.as_op().unwrap().apply(
                prev_token.as_nr(),
                data.get(idx + 1).unwrap_or(&Token::Invalid).clone().as_nr(),
                data.get(idx + 2).unwrap_or(&Token::Invalid).clone().as_nr(),
            );
            ret.push(x.clone().ok_or("Applying operation failed".to_string())?);
            // println!("After _parse: {x:?}");
            if i.as_op().unwrap().is_forward() {
                idx += 3;
            } else {
                idx += 2;
            }
            break;
        }
        prev_token = i.clone();
        idx += 1;
    }

    ret.extend_from_slice(&data[idx..]);
    if ret.len() != 1 {
        return parse(ret);
    }

    Ok(ret)
}

// fn _parse(op: Op, data: Vec<Token>) -> Token {
//     op.apply(data.iter().map(|x| x.as_nr()).collect())
// }
