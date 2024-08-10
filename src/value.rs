pub type Double = f64;
pub type Line = u16;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(Double),
}
