use tokeniser::tokenise;

use crate::{parser::Parse, tokeniser::*};

mod parser;
mod tokeniser;

fn main() {
    // println!("{}", dbg_tokenise("20 * 12"));
    // println!("{:?}", tokenise("20 * 12").parse());
    // println!("{}", dbg_tokenise("123123123123123.0 / 123"));
    // println!("{:?}", tokenise("123123123123123.0 / 123").parse());

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let line = line.trim();

    println!("{}", dbg_tokenise(line));
    println!("{:?}", tokenise(line).parse());
}
