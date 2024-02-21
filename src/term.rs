use std::fmt::Arguments;
use std::io::Read;
use std::io::Write;
use termion::raw::RawTerminal;

use termion::raw::IntoRawMode;

type Buffer = [u8; 8];
type StringResult = Result<Option<String>, Box<dyn std::error::Error>>;

macro_rules! term_write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write(format_args!($($arg)*))
    };
}

pub struct Term {
    line: String,
    history: Vec<String>,
    max_hist_len: usize,
    hist_idx: usize,
    use_hist: bool,
    stdout: RawTerminal<std::io::Stdout>,
    stdin: std::io::Stdin,
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
            stdin: std::io::stdin(),
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
                        self.add_line_to_history();
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

    fn add_line_to_history(&mut self) {
        if self.history.len() >= self.max_hist_len {
            self.history.remove(0);
        }
        self.history.push(self.line.clone());
        self.hist_idx = 0;
        self.use_hist = false;
        self.line.clear();
    }

    fn parse_char(&mut self, buf: Buffer) -> StringResult {
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
                    self.add_line_to_history();
                    // print!("\r\n");
                    return Ok(Some(ret));
                }
                // print!("{ch}");
                self.line += ch.to_string().as_str();
            } else {
                self.handle_char(ch);
            }
        }
        Ok(None)
    }

    pub fn write(&mut self, data: Arguments<'_>) -> std::io::Result<()> {
        self.stdout.write_fmt(data)
    }

    pub fn next(&mut self) -> StringResult {
        let mut buf = Buffer::default();
        self.stdin.read(&mut buf)?;
        let ret = self.parse_char(buf)?;
        if !self.line.is_empty() {
            term_write!(
                self,
                "{}\r>{}",
                termion::clear::CurrentLine,
                self.line.clone()
            )?;
        }
        Ok(ret)
    }
}
