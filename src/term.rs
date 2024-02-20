use std::io::Read;
use std::io::Write;
use termion::raw::RawTerminal;

use termion::raw::IntoRawMode;

pub struct Term {
    line: String,
    history: Vec<String>,
    max_hist_len: usize,
    hist_idx: usize,
    use_hist: bool,
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
            history: Vec::new(),
            max_hist_len: 10,
            hist_idx: 0,
            use_hist: false,
            stdout: std::io::stdout().into_raw_mode().unwrap(),
        }
    }

    fn update_history(&mut self) {
        if self.use_hist && self.hist_idx < self.history.len() {
            let idx = if self.history.len() == 1 {
                0
            } else {
                self.history.len() - self.hist_idx - 1
            };
            self.line = self.history.get(idx).unwrap().clone();
        }
    }

    fn handle_char(&mut self, ch: char) {
        // print!("\\u{{{:x}}}\r\n", ch as u32);
        match ch {
            //Backspace
            '\u{7f}' => {
                if !self.line.is_empty() {
                    self.line.remove(self.line.len() - 1);
                }
            }
            //Arrow Up
            '\u{b7}' => {
                if !self.use_hist {
                    self.use_hist = true;
                    if !self.line.is_empty() {
                        self.history.push(self.line.clone());
                        self.hist_idx += 1
                    }
                } else if self.hist_idx + 1 < self.history.len() {
                    self.hist_idx += 1
                }
                self.update_history()
            }
            //Arrow Down
            '\u{b8}' => {
                if !self.use_hist {
                    self.use_hist = true;
                } else if self.hist_idx > 0 {
                    self.hist_idx -= 1
                }
                self.update_history()
            }
            _ => {}
        }
    }

    pub fn next(&mut self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut buf = [0, 0, 0, 0, 0, 0, 0, 0];
        let mut stdin = std::io::stdin();
        stdin.read(&mut buf)?;
        let nr = buf.iter().map(|x| *x as u32).reduce(|x, y| x + y).unwrap();
        let ch = char::from_u32(nr);
        // print!("{buf:?} {nr:?} {ch:?}\r\n");
        if buf == [3, 0, 0, 0, 0, 0, 0, 0] {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                String::from("Ctrl-C"),
            )));
        } else if let Some(ch) = ch {
            if ch.is_ascii_alphanumeric() || ch.is_ascii_whitespace() || ch.is_ascii_punctuation() {
                if ch == '\r' {
                    let ret = self.line.clone();
                    if self.history.len() >= self.max_hist_len {
                        self.history.remove(0);
                    }
                    self.history.push(ret.clone());
                    self.hist_idx = 0;
                    self.use_hist = false;
                    self.line.clear();
                    // print!("\r\n");
                    return Ok(Some(ret));
                }
                // print!("{ch}");
                self.line += ch.to_string().as_str();
            } else {
                self.handle_char(ch);
            }
        }
        print!("{}", termion::clear::CurrentLine);
        print!("\r>{}", self.line);
        Ok(None)
    }
}
