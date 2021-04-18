use std::env;
use std::fs::File;
use std::io;
use std::process;
use std::str;

use io::{Read, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Rust std env args, 0 : executable file name, 1.. : arguments
    match args.len() {
        2 => run_file(&args[1]),
        1 => run_prompt(),
        _ => {
            println!("Usage: rlox [script]");
            process::exit(64)
        }
    }
}

fn run_file(path: &str) {
    let mut file = File::open(path).unwrap();
    let mut bytes = vec![];

    file.read_to_end(&mut bytes).unwrap();

    // Default charset : UTF-8
    run(str::from_utf8(&bytes).unwrap())
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        run(&line);
    }
}

fn run(source: &str) {
    // TODO : Scan tokens
    println!("{}", source);
}
