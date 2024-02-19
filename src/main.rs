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
    let term = Term::new();
    loop {
        print!(">");
        // println!("{}", dbg_tokenize(line));
        let line = match term.next() {
            Ok(o) => o,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };
        let out = dbg_tokenize(&line).parse();
        match out {
            Ok(o) => {
                println!("={o}");
            }
            Err(e) => {
                println!("Error: {e}");
            }
        }
    }
}
