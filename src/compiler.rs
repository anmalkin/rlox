use crate::error::RloxResult;
use crate::scanner::Scanner;

pub fn compile(source: String) -> RloxResult {
    let mut scanner = Scanner::new(&source);
    scanner.scan_token();
    todo!()
}
