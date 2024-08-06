#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

mod chunk;
mod compiler;
mod error;
mod scanner;
mod value;
mod vm;

use std::env;
use std::io;

use vm::VM;

use crate::error::RloxResult;

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        match repl() {
            Ok(()) => {}
            Err(e) => println!("Error: {e}"),
        }
    } else if args.len() == 2 {
        args.next();
        match run_file(args.next().unwrap()) {
            Ok(()) => {}
            Err(e) => println!("Error: {e}"),
        }
    } else {
        eprintln!("Usage: rlox [path]");
        std::process::exit(64);
    }
}

fn repl() -> RloxResult {
    loop {
        let mut vm = VM::new();
        let mut line = String::new();
        print!("rlox > ");
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        vm.interpret(&line)?;
    }
    Ok(())
}

fn run_file(path: String) -> RloxResult {
    let mut vm = VM::new();
    let contents = std::fs::read_to_string(path)?;
    vm.interpret(&contents)
}
