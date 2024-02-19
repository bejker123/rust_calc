use crate::{parser::Parse, rational::*, tokenizer::*};
use std::io::Write;

mod op;
mod parser;
mod rational;
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
    loop {
        let mut line = String::new();
        print!(">");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        // println!("{}", dbg_tokenize(line));
        let out = dbg_tokenize(line).parse();
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
