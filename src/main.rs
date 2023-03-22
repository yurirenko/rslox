mod expression;
mod token;
mod scanner;

use std::io::{stdin, stdout, BufRead, Write};
use std::{env, error, fs, process};
use crate::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        println!("Usage: rslox [script]");
        process::exit(64);
    } else if args.len() == 1 {
        if let Err(err) = run_file(&args[0]) {
            println!("Execution failed! {}", err);
            process::exit(1);
        }
    } else {
        if let Err(err) = run_prompt() {
            println!("Execution failed! {}", err);
            process::exit(1);
        }
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn error::Error>> {
    let file_bytes = fs::read(path)?;
    let script_contents = String::from_utf8(file_bytes)?;

    run(&script_contents);
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn error::Error>> {
    let stdin = stdin();
    let mut line = String::new();

    loop {
        line.clear();
        print!("> ");
        stdout().flush()?;

        let mut handle = stdin.lock();

        handle.read_line(&mut line)?;

        if line.trim().is_empty() {
            process::exit(0);
        }

        run(&line);
    }
}

fn run(program_contents: &str) {
    let tokens = Scanner::init(&program_contents)
        .scan_tokens();
    println!("Tokens: {:?}", tokens);
}
