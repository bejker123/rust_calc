use std::io::Write;

use term::Term;

use crate::{parser::Parse, rational::*, tokenizer::*};

mod op;
mod parser;
mod rational;
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
    print!(">");
    loop {
        // if new_line {
        //     print!(">");
        //     new_line = false;
        // }
        std::io::stdout().flush().unwrap();
        let line = match term.next() {
            Ok(o) => o,
            Err(e) => {
                print!("Error: {e}\r\n");
                break;
            }
        };
        if let Some(line) = line {
            print!("\r\n");
            let out = dbg_tokenize(&line).parse();
            match out {
                Ok(o) => {
                    print!("={o}\r\n");
                }
                Err(e) => {
                    print!("Error: {e}\r\n");
                }
            }
            print!("\r>");
        }
    }
}
