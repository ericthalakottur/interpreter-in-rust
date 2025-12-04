use std::io::{self, Write};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let file_path = &args[1];
        let contents = fs::read_to_string(file_path)
            .expect("The first argument should point to a readable file");
        dbg!(contents);
    } else {
        let mut line = String::new();
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
            line.clear();
        }
    }
}
