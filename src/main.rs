use tokenizer::tokenize;

use crate::{parser::Parse, tokenizer::*};

mod parser;
mod tokenizer;

fn test(s: &str) {
    println!("{}", dbg_tokenize(s));
    println!("{:?}", tokenize(s).parse());
}

fn main() {
    // println!("{}", dbg_tokenize("20 / 12"));
    // println!("{:?}", tokenize("20 * 12").parse());

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        // println!("{}", dbg_tokenize(line));
        let out = tokenize(line).parse().unwrap();
        println!("{:?}", out.first().unwrap().as_nr().unwrap());
    }
}
