use tokenizer::tokenize;

use crate::{
    parser::{Op, Parse},
    tokenizer::*,
};

mod parser;
mod tokenizer;

fn main() {
    // println!(
    //     "{}",
    //     Op::Mul(Box::new(Op::Number(123.0)), Box::new(Op::Number(123.0)))
    //         .apply()
    //         .expect_err("")
    // );
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        // println!("{}", dbg_tokenize(line));
        let out = tokenize(line).parse().unwrap();
        println!("{}", out.first().map_or(0.0, |x| x.as_nr().unwrap_or(0.0)));
    }
}
