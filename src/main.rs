#![warn(clippy::pedantic, clippy::nursery)]

mod chunk;
mod value;

use chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    let pos = chunk.add_constant(1.2);
    chunk.write(OpCode::Constant(pos), 123);
    chunk.write(OpCode::Return, 123);
    println!("{chunk}");
}
