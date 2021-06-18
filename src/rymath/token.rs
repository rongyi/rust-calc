#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
    Num(f64),
    EOF,
}
