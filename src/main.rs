use std::io::Write;

use term::Term;

use crate::{parser::Parse, rational::*, tokenizer::*};

mod op;
mod parser;
mod rational;
#[macro_use]
mod term;
mod tokenizer;

fn main() {
    // println!("{}", dbg_tokenize("1+2/2"));
    // println!("{:?}", tokenize("1+2/2").parse());
    // println!("{}", dbg_tokenize("5/5^2"));
    // println!("{:?}", tokenize("5/5^2").parse());
    // println!(
    //     "{}",
    //     Op::Mul(Box::new(Op::Number(123.0)), Box::new(Op::Number(123.0)))
    //         .apply()
    //         .expect_err("")
    // );
    let mut term = Term::new();
    // let mut new_line = true;
    term_write!(term, ">").unwrap();
    loop {
        std::io::stdout().flush().unwrap();
        let line = match term.next() {
            Ok(o) => o,
            Err(e) => {
                term_write!(term, "Error: {e}\r\n").unwrap();
                break;
            }
        };
        if let Some(line) = line {
            print!("\r\n");
            let out = dbg_tokenize(&line).parse();
            match out {
                Ok(o) => {
                    term_write!(term, "={o}\r\n").unwrap();
                }
                Err(e) => {
                    term_write!(term, "Error: {e}\r\n").unwrap();
                }
            }
            term_write!(term, "\r>").unwrap();
        }
    }
}
