mod environment;
mod expression;
mod interpreter;
mod parser;
mod scanner;
mod statement;
mod token;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::io::{stdin, stdout, BufRead, Write};
use std::{env, error, fs, process};

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
    } else if let Err(err) = run_prompt() {
        println!("Execution failed! {}", err);
        process::exit(1);
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn error::Error>> {
    let file_bytes = fs::read(path)?;
    let script_contents = String::from_utf8(file_bytes)?;
    let mut interpreter = Interpreter::new();

    run(&script_contents, &mut interpreter);
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn error::Error>> {
    let stdin = stdin();
    let mut line = String::new();
    let mut interpreter = Interpreter::new();

    loop {
        line.clear();
        print!("> ");
        stdout().flush()?;

        let mut handle = stdin.lock();

        handle.read_line(&mut line)?;

        if line.trim().is_empty() {
            process::exit(0);
        }

        run(&line, &mut interpreter);
    }
}

fn run(program_contents: &str, interpreter: &mut Interpreter) {
    let tokens = Scanner::init(program_contents).scan_tokens();
    let mut parser = Parser::init(&tokens);
    let statements = parser.parse();

    interpreter.interpret(statements);
}
