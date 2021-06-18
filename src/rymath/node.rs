#[derive(Debug, Clone)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Neg(Box<Node>),
    Pow(Box<Node>, Box<Node>),
    Num(f64),
}

pub fn eval(expr: Node) -> Result<f64, String> {
    match expr {
        Node::Num(i) => Ok(i),
        Node::Add(l, r) => Ok(eval(*l)? + eval(*r)?),
        Node::Sub(l, r) => Ok(eval(*l)? - eval(*r)?),
        Node::Mul(l, r) => Ok(eval(*l)? * eval(*r)?),
        Node::Div(l, r) => Ok(eval(*l)? / eval(*r)?),
        Node::Pow(l, r) => Ok(eval(*l)?.powf(eval(*r)?)),
        Node::Neg(i) => Ok(-eval(*i)?),
    }
}
