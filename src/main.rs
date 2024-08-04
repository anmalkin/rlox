#![warn(clippy::pedantic, clippy::nursery)]

mod chunk;
mod value;
mod vm;

use chunk::{Chunk, OpCode};
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    let a = chunk.add_constant(1.2);
    let b = chunk.add_constant(2.5);
    chunk.write(OpCode::Constant(a), 123);
    chunk.write(OpCode::Constant(b), 123);
    chunk.write(OpCode::Add, 123);
    chunk.write(OpCode::Return, 123);
    vm.interpret(&chunk);
}
