use term::Term;

use crate::{parser::Parse, rational::*, tokenizer::*};

mod op;
mod parser;
mod rational;
#[macro_use]
mod term;
mod tokenizer;

fn main() {
    let mut term = Term::new();
    // let mut new_line = true;
    term_write!(term, ">").unwrap();
    loop {
        term.flush().unwrap();
        let line = match term.next() {
            Ok(o) => o,
            Err(e) => {
                term_write!(term, "Error: {e}\r\n").unwrap();
                break;
            }
        };
        if let Some(line) = line {
            term_write!(term, "\r\n").unwrap();
            let (line, opts) = pre_tokenize(&line);
            let out = if opts.debug {
                dbg_tokenize(line).parse()
            } else {
                tokenize(line).parse()
            };
            match out {
                Ok(o) => {
                    if opts.as_float {
                        term_write!(term, "={}\r\n", o.to_float()).unwrap();
                    } else {
                        term_write!(term, "={o}\r\n").unwrap();
                    }
                }
                Err(e) => {
                    term_write!(term, "Error: {e}\r\n").unwrap();
                }
            }
            term_write!(term, "\r>").unwrap();
        }
    }
}
