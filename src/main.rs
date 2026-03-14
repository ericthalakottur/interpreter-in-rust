use std::io::{self, Write};
use std::{env, fs};

use interpreter_in_rust::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1];
        let contents = fs::read_to_string(file_path)
            .expect("The first argument should point to a readable file");
        dbg!(contents);
    } else {
        let mut line = String::new();
        let mut scanner = Scanner::new();
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            match io::stdin().read_line(&mut line) {
                Ok(_) => (),
                Err(err) => println!("Error: {}", err),
            };
            line.pop();
            if line == "exit" {
                break;
            }

            dbg!(&line);
            scanner.scan_tokens(&line);

            line.clear();
        }
    }
}

fn error(line: u8, message: &str) {
    report(line, "", message);
}

fn report(line: u8, location: &str, message: &str) {
    eprintln!("[line {} \"] Error {}: {}", line, location, message);
}
