use std::{env, fs, io::{self, Write}};
use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Lox {had_error: false}
    }

    pub fn main(&mut self) {
        let args: Vec<String> = env::args().collect();

        if args.len() > 2 {
            println!("Usage: loxit [script]");
        } else if args.len() == 2 {
            self.run_file(&args[1]);
        } else {
            self.run_prompt();
        }
    }

    fn run_file(&mut self, path: &str) {
        let source = fs::read_to_string(path).unwrap();
        self.run(&source);
    }
    
    fn run_prompt(&mut self) {
        let mut input = String::new();
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        loop {
            print!("> ");
            stdout.flush().unwrap();
            stdin.read_line(&mut input).unwrap();
            if input.trim().is_empty() {
                break
            }
            self.run(&input);
            input.clear();
        }
    }

    fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
    
        for token in tokens {
            println!("TOK: {}", token);
        }
    }
    
    
    fn error(&mut self, line: isize, message: &str) {
        self.report(line, "", message);
    }
    
    fn report(&mut self, line: isize, where_str: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, where_str, message);
        self.had_error = true;
    }
}
