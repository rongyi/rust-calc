#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Token {
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
    Caret,
    LeftParen,
    RightParen,
    EOF,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum OpOrder {
    DefaultOpOrder,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

impl Token {
    pub fn order(&self) -> OpOrder {
        use OpOrder::*;
        use Token::*;
        match *self {
            Add | Sub => AddSub,
            Mul | Div => MulDiv,
            Caret => Power,
            // Negative is manual order
            _ => DefaultOpOrder,
        }
    }
}
