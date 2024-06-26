use io::IsTerminal;
use io::Read;
use io::Write;
use std::fmt::Arguments;
use std::io;
use termion::cursor::DetectCursorPos;
use termion::raw::RawTerminal;

use termion::raw::IntoRawMode;

type Buffer = [u8; 8];
type StringResult = Result<Option<String>, Box<dyn std::error::Error>>;

macro_rules! term_write {
    ($dst:expr, $($arg:tt)*) => {
        $dst.write(format_args!($($arg)*))
    };
}

macro_rules! term_writeln {
    ($dst:expr, $($arg:tt)*) => {
        $dst.writeln(format_args!($($arg)*))
    };
}

enum StdoutOpt {
    Stdout(io::Stdout),
    Raw(RawTerminal<io::Stdout>),
}

impl io::Write for StdoutOpt {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            StdoutOpt::Raw(x) => x.write(buf),
            StdoutOpt::Stdout(x) => x.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            StdoutOpt::Raw(x) => x.flush(),
            StdoutOpt::Stdout(x) => x.flush(),
        }
    }
}

impl StdoutOpt {
    pub fn new() -> Self {
        if io::stdout().is_terminal() {
            StdoutOpt::Raw(io::stdout().into_raw_mode().unwrap())
        } else {
            StdoutOpt::Stdout(io::stdout())
        }
    }

    pub fn lock(&mut self) -> io::StdoutLock<'_> {
        match self {
            StdoutOpt::Raw(x) => x.lock(),
            StdoutOpt::Stdout(x) => x.lock(),
        }
    }

    pub fn suspend_raw_mode(&mut self) -> Result<(), io::Error> {
        match self {
            StdoutOpt::Raw(x) => x.suspend_raw_mode(),
            StdoutOpt::Stdout(_) => Ok(()),
        }
    }
}

pub struct Term {
    line: String,
    history: Vec<String>,
    max_hist_len: usize,
    hist_idx: usize,
    use_hist: bool,
    cur_pos: u16,
    stdout: StdoutOpt,
    stdin: std::fs::File,
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
            cur_pos: 0,
            stdout: StdoutOpt::new(),
            stdin: termion::get_tty().unwrap(),
        }
    }

    pub fn read_pipe(&mut self) -> Option<String> {
        let stdin = io::stdin();
        if stdin.is_terminal() {
            None
        } else {
            let mut stdin = stdin.lock();

            let mut line = String::new();

            // Could also `match` on the `Result` if you wanted to handle `Err`
            while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
                if n_bytes == 0 {
                    break;
                }
            }
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
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
            self.cur_pos = self.line.len() as u16;
        }
    }

    fn handle_char(&mut self, ch: char) {
        // print!("\\u{{{:x}}}\r\n", ch as u32);
        match ch {
            //Backspace
            '\u{7f}' => {
                if !self.line.is_empty() && self.cur_pos > 0 {
                    self.line.remove(self.cur_pos as usize - 1);
                    self.cur_pos -= 1;
                }
            }
            //Delete
            '\u{127}' => {
                if !self.line.is_empty() && self.cur_pos < self.line.len() as u16 {
                    self.line.remove(self.cur_pos as usize);
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
            //Arrow Left
            '\u{ba}' => {
                if self.cur_pos > 0 {
                    self.cur_pos -= 1;
                }
            }
            //Arrow Right
            '\u{b9}' => {
                if self.cur_pos < self.line.len() as u16 {
                    self.cur_pos += 1;
                }
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
        self.cur_pos = 0;
    }

    fn parse_char(&mut self, buf: Buffer) -> StringResult {
        let nr = buf
            .iter()
            .map(|x| *x as u32)
            .reduce(|x, y| x + y)
            .ok_or(io::Error::new(
                io::ErrorKind::Other,
                String::from("Failed to reduce bytes to u32."),
            ))?;
        let ch = char::from_u32(nr);
        // print!("{buf:?} {nr:?} {ch:?}\r\n");
        if buf == [3, 0, 0, 0, 0, 0, 0, 0] {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Interrupted,
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
                self.line.insert(self.cur_pos as usize, ch);
                self.cur_pos += 1;
            } else {
                self.handle_char(ch);
            }
        }
        Ok(None)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }

    pub fn write(&mut self, data: Arguments<'_>) -> io::Result<()> {
        self.stdout.write_fmt(data)
    }

    pub fn writeln(&mut self, data: Arguments<'_>) -> io::Result<()> {
        self.stdout.write_fmt(data)?;
        if io::stdout().is_terminal() {
            self.stdout.write(b"\r")?;
        }
        let _ = self.stdout.write(b"\n")?;
        Ok(())
    }

    pub fn next(&mut self) -> StringResult {
        let mut buf = Buffer::default();
        self.stdin.read(&mut buf)?;
        let ret = self.parse_char(buf)?;
        let change_pos = if let Ok(mut pos) = self.stdout.cursor_pos() {
            pos.0 = self.cur_pos + 2;
            termion::cursor::Goto(pos.0, pos.1).to_string()
        } else {
            String::new()
        };
        if ret.is_none() {
            term_write!(
                self,
                "{}\r>{}{}",
                termion::clear::CurrentLine,
                self.line.clone(),
                change_pos,
            )?;
        }
        Ok(ret)
    }
}
