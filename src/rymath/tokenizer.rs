use super::token::Token;
use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(expr: &'a str) -> Self {
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
    fn test_todo() {
        let mut t = Tokenizer::new("+");
        assert_eq!(t.next(), None);
    }
}
