use std::io::Read;
use std::io::Write;
use termion::raw::RawTerminal;

use termion::raw::IntoRawMode;

pub struct Term {
    line: String,
    stdout: RawTerminal<std::io::Stdout>,
}

impl Drop for Term {
    fn drop(&mut self) {
        self.stdout.lock().flush().unwrap();
        self.stdout.suspend_raw_mode().unwrap();
    }
}

impl Term {
    pub fn new() -> Self {
        Self {
            line: String::new(),
            stdout: std::io::stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn next(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut buf = [0, 0, 0, 0];
        let mut stdin = std::io::stdin();
        stdin.read(&mut buf)?;
        // print!("{buf:?}\r\n");
        let ch = char::from_u32(u32::from_ne_bytes(buf.try_into().unwrap()));
        // print!("{ch:?}\r\n");
        if buf == [3, 0, 0, 0] {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                String::from("Ctrl-C"),
            )));
        }
        if let Some(ch) = ch {
            if ch == '\r' {
                let ret = self.line.clone();
                self.line.clear();
                print!("\r\n");
                return Ok(Some(ret));
            }
            print!("{ch}");
            self.line += ch.to_string().as_str();
        }
        Ok(None)
    }
}
