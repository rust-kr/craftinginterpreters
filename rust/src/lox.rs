use std::fs::File;
use std::io;
use std::str;

use io::{Read, Write};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) -> i32 {
        let mut file = File::open(path).unwrap();
        let mut bytes = vec![];

        file.read_to_end(&mut bytes).unwrap();

        // Default charset : UTF-8
        self.run(str::from_utf8(&bytes).unwrap());

        if self.had_error {
            return 65;
        }

        0
    }

    pub fn run_prompt(&mut self) -> i32 {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            if line.trim().is_empty() {
                break;
            }

            self.run(&line);
            self.had_error = false;
        }

        0
    }

    fn run(&mut self, source: &str) {
        // TODO : Scan tokens
        println!("{}", source);
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, _where: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, _where, message);
        self.had_error = true
    }
}
