#![warn(clippy::pedantic, clippy::nursery)]

mod chunk;
mod value;

use chunk::Chunk;

fn main() {
    let chunk = Chunk::new();
    println!("{chunk:#?}");
}
