use std::error::Error;

use calc_core::{
    parser::{KnownLiterals, Parse},
    tokenizer::{dbg_tokenize, pre_tokenize, tokenize},
};

use crate::term::Term;

#[macro_use]
mod term;

fn main() -> Result<(), Box<dyn Error>> {
    let mut term = Term::new();
    // let mut new_line = true;
    let mut known_literals = KnownLiterals::new();
    if let Some(pipe) = term.read_pipe() {
        let pipe = pipe.trim();
        term_writeln!(term, "\r{}", tokenize(pipe).parse(&mut known_literals)?)?;
        return Ok(());
    }
    term_write!(term, ">")?;

    loop {
        term.flush()?;
        let line = match term.next() {
            Ok(o) => o,
            Err(e) => {
                term_writeln!(term, "Error: {e}")?;
                break;
            }
        };
        if let Some(line) = line {
            term_writeln!(term, "")?;
            let (line, opts) = pre_tokenize(&line);
            let out = if opts.debug {
                dbg_tokenize(line).parse(&mut known_literals)
            } else {
                tokenize(line).parse(&mut known_literals)
            };
            match out {
                Ok(o) => {
                    if opts.as_float {
                        term_writeln!(term, "={}", o.to_float())?;
                    } else {
                        term_writeln!(term, "={o}")?;
                    }
                }
                Err(e) => {
                    term_writeln!(term, "Error: {e}")?;
                }
            }
            term_write!(term, "\r>")?;
        }
    }

    Ok(())
}
