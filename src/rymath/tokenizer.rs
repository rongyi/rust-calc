use super::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &'a str) -> Self {
        Tokenizer {
            expr: expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let n = self.expr.next();
        match n {
            Some('0'..='9') => {
                let mut num = n?.to_string();
                while let Some(n) = self.expr.peek() {
                    if n.is_numeric() || n == &'.' {
                        num.push(self.expr.next()?);
                    } else {
                        break;
                    }
                }
                Some(Token::Num(num.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Sub),
            Some('*') => Some(Token::Mul),
            Some('/') => Some(Token::Div),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token() {
        let mut t = Tokenizer::new("3.14");
        assert_eq!(t.next().unwrap(), Token::Num(3.14));

        let mut t = Tokenizer::new("10");
        assert_eq!(t.next().unwrap(), Token::Num(10.0));
    }

    #[test]
    fn test_empty() {
        let mut t = Tokenizer::new("");
        assert_eq!(t.next().unwrap(), Token::EOF);
    }

    #[test]
    fn test_add_sub() {
        let mut t = Tokenizer::new("+-*/^()");
        assert_eq!(t.next().unwrap(), Token::Add);
        assert_eq!(t.next().unwrap(), Token::Sub);
        assert_eq!(t.next().unwrap(), Token::Mul);
        assert_eq!(t.next().unwrap(), Token::Div);
        assert_eq!(t.next().unwrap(), Token::Caret);
        assert_eq!(t.next().unwrap(), Token::LeftParen);
        assert_eq!(t.next().unwrap(), Token::RightParen);
        assert_eq!(t.next().unwrap(), Token::EOF);
    }
}
