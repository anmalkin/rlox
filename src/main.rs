#![warn(clippy::pedantic, clippy::nursery)]
// #![allow(dead_code)]

mod chunk;
mod compiler;
mod error;
mod scanner;
mod value;
mod vm;

use std::env;

use vm::VM;

use crate::error::RloxResult;

fn main() {
    // TODO: Add back REPL support
    let mut args = env::args();
    if args.len() == 2 {
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

fn run_file(path: String) -> RloxResult {
    let mut vm = VM::new();
    let contents = std::fs::read_to_string(path)?;
    vm.interpret(&contents)
}
