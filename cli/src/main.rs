use calc_core::{
    parser::{KnownLiterals, Parse},
    tokenizer::{dbg_tokenize, pre_tokenize, tokenize},
};

use crate::term::Term;

#[macro_use]
mod term;

fn main() {
    let mut term = Term::new();
    // let mut new_line = true;
    let mut known_literals = KnownLiterals::new();
    if let Some(pipe) = term.read_pipe() {
        let pipe = pipe.trim();
        term_write!(
            term,
            "\r{}\r\n",
            tokenize(pipe).parse(&mut known_literals).unwrap()
        )
        .unwrap();
        return;
    }
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
                dbg_tokenize(line).parse(&mut known_literals)
            } else {
                tokenize(line).parse(&mut known_literals)
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
