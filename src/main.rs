use tokeniser::tokenise;

use crate::{parser::Parse, tokeniser::*};

mod parser;
mod tokeniser;

fn test(s: &str) {
    println!("{}", dbg_tokenise(s));
    println!("{:?}", tokenise(s).parse());
}

fn main() {
    // println!("{}", dbg_tokenise("20 / 12"));
    // println!("{:?}", tokenise("20 * 12").parse());

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let line = line.trim();

        // println!("{}", dbg_tokenise(line));
        let out = tokenise(line).parse().unwrap();
        println!("{:?}", out.first().unwrap().as_nr().unwrap());
    }
}
