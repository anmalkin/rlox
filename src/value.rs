pub type Double = f64;
pub type Line = u16;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum ObjectType {
    String(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(Double),
    Object(ObjectType),
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Constant<'src> {
    String(&'src str),
    Number(Double),
}

// TODO: Implement object type to enable garbage collection
#[derive(Debug)]
struct _Object<T> {
    kind: ObjectType,
    next: *mut T,
}
