use std::env;
use std::process;

mod lox;

fn main() {
    let mut lox = lox::Lox::new();
    let args: Vec<String> = env::args().collect();

    // Rust std env args, 0 : executable file name, 1.. : arguments
    let exit_code = match args.len() {
        2 => lox.run_file(&args[1]),
        1 => lox.run_prompt(),
        _ => {
            println!("Usage: rlox [script]");
            64 // Help exit code it 64
        }
    };

    process::exit(exit_code);
}
