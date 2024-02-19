use std::io::Write;

pub struct Term {}

impl Term {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next(&self) -> std::io::Result<String> {
        let mut line = String::new();
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut line)?;
        Ok(line.trim().to_string())
    }
}
