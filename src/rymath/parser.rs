use super::node::Node;
use super::token::OpOrder;
use super::token::Token;
use super::tokenizer::Tokenizer;

pub struct Parser<'a> {
    lexer: Tokenizer<'a>,
    cur_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Parser, String> {
        let mut l = Tokenizer::new(expr);
        let cur = match l.next() {
            Some(t) => t,
            None => return Err("invalid input".to_string()),
        };

        Ok(Parser {
            lexer: l,
            cur_token: cur,
        })
    }
    pub fn parse(&mut self) -> Result<Node, String> {
        self.gen_node(OpOrder::DefaultOpOrder)
    }
}

impl<'a> Parser<'a> {
    fn next(&mut self) -> Result<(), String> {
        let n = match self.lexer.next() {
            Some(t) => t,
            None => return Err("invalid input".to_string()),
        };
        self.cur_token = n;
        Ok(())
    }
    fn check_paren(&mut self) -> Result<(), String> {
        if self.cur_token != Token::RightParen {
            return Err("( not match".to_string());
        }
        self.next()?;
        Ok(())
    }

    fn parse_left(&mut self) -> Result<Node, String> {
        let cur = self.cur_token.clone();
        match cur {
            Token::Num(i) => {
                self.next()?;
                Ok(Node::Num(i))
            }
            Token::Sub => {
                self.next()?;
                let expr = self.gen_node(OpOrder::Negative)?;
                Ok(Node::Neg(Box::new(expr)))
            }
            Token::LeftParen => {
                self.next()?;
                let l = self.gen_node(OpOrder::DefaultOpOrder)?;
                self.check_paren()?;
                if self.cur_token == Token::LeftParen {
                    let r = self.gen_node(OpOrder::MulDiv)?;
                    return Ok(Node::Mul(Box::new(l), Box::new(r)));
                }
                Ok(l)
            }
            _ => return Err("invalid Token".to_string()),
        }
    }
    fn gen_node(&mut self, prev_order: OpOrder) -> Result<Node, String> {
        let mut l = self.parse_left()?;

        while prev_order < self.cur_token.order() {
            if self.cur_token == Token::EOF {
                break;
            }
            // cur stop at op(+ - * ... )
            let node = self.combine_op(l.clone())?;
            l = node;
        }
        Ok(l)
    }

    fn combine_op(&mut self, l: Node) -> Result<Node, String> {
        match self.cur_token {
            Token::Add => {
                self.next()?;
                let r = self.gen_node(OpOrder::AddSub)?;
                Ok(Node::Add(Box::new(l), Box::new(r)))
            }

            Token::Sub => {
                self.next()?;
                let r = self.gen_node(OpOrder::AddSub)?;
                Ok(Node::Sub(Box::new(l), Box::new(r)))
            }
            Token::Mul => {
                self.next()?;
                let r = self.gen_node(OpOrder::MulDiv)?;
                Ok(Node::Mul(Box::new(l), Box::new(r)))
            }

            Token::Div => {
                self.next()?;
                let r = self.gen_node(OpOrder::MulDiv)?;
                Ok(Node::Div(Box::new(l), Box::new(r)))
            }

            Token::Caret => {
                self.next()?;
                let r = self.gen_node(OpOrder::Power)?;
                Ok(Node::Pow(Box::new(l), Box::new(r)))
            }
            _ => return Err("no valid combine op".to_string()),
        }
    }
}
