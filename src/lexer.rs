use std::{slice::Iter, iter::Peekable};
use anyhow::{Result, anyhow};
use logos::Logos;

pub fn from_logos(lexer: &mut logos::Lexer<Token>) -> (Vec<Token>, Vec<String>) {
    let mut lexer_tokens:Vec<Token> = Vec::new();
    let mut lexer_slices:Vec<String> = Vec::new();
    loop {
        match lexer.next(){
            Some(Token::Error) => {continue;},
            Some(x) => {
                lexer_tokens.push(x);
                lexer_slices.push(lexer.slice().to_owned());
            },
            None => {break;},
        } 
    };
 
    (lexer_tokens, lexer_slices)
}

#[derive(Debug,PartialEq)]
pub enum Expression {
    Numeric(f64),
    Binary(Box<Expression>, Token, Box<Expression>),
    LeftUnary(String, Box<Expression>),
    Parenthesis(Box<Expression>),
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Divide,
    #[token("*")]
    Multiply,
    #[token("^")]
    Exponentiate,
    #[token("(", priority = 5)]
    LeftParenthesis,
    #[token(")")]
    RightParenthesis,
    #[token("pi", |_| std::f64::consts::PI)]
    PI(f64),
    #[token("e", |_| std::f64::consts::E)]
    EulersConstant(f64),
    #[token("=")]
    Equals,
    #[regex(r"[a-zA-Z]+\(", priority = 10)]
    FunctionCall,
    #[regex("([0-9]*[.])?[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Numeric(f64),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip, priority = 1)]
    Error,
}

impl Token {
    fn is_infix(&self) -> bool{
        match *self {
            Token::Plus => true,
            Token::Minus => true,
            Token::Multiply => true,
            Token::Divide => true,
            Token::Exponentiate => true,
            _ => false
        }
    }

    fn left_binding_power(&self) -> u32 {
        match *self {
            Token::Plus => 10,
            Token::Minus => 10,
            Token::Multiply => 20,
            Token::Divide => 20,
            Token::Exponentiate => 50,
            Token::LeftParenthesis => 100,
            Token::FunctionCall => 99,
            Token::RightParenthesis => 0,
            _ => 0
        }
    }
    
    fn null_denotion(&self) -> Result<Expression> {
        match *self {
            Token::Numeric(f) => Ok(Expression::Numeric(f)),
            Token::PI(f) => Ok(Expression::Numeric(f)),
            Token::EulersConstant(f) => Ok(Expression::Numeric(f)),
            _ => Err(anyhow!("expecting literal"))
	    }
    }

    fn left_denotion(&self, parser: &mut Parser, lhs: Expression) -> Result<Expression> {
        if self.is_infix() {
            let rhs = parser.expression(self.left_binding_power())?;
            Ok(Expression::Binary(Box::new(lhs), self.clone(), Box::new(rhs)))
        }else{
            Err(anyhow!("expecting operator"))
        }
    }

}

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
    slices: Peekable<Iter<'a, String>>
}

impl<'a> Parser<'a> {
    fn next(&mut self){
        self.tokens.next();
        self.slices.next();
    }

    pub fn new(tokens: Iter<'a, Token>, slices: Iter<'a, String>) -> Self {
        Parser{ tokens: tokens.peekable(), slices: slices.peekable() }
    }

    pub fn expression(&mut self, rbp: u32) -> Result<Expression> {
        let mut was_paren = false;
        let mut left:Expression = match self.tokens.peek(){
            Some(Token::FunctionCall) => {
                match self.slices.next() {
                    Some(x) => {
                        self.tokens.next();
                        was_paren = true;
                        Expression::LeftUnary(x.clone(), Box::new(self.expression(0)?))
                    },
                    _ => anyhow::bail!("no slice to define function".to_string()) 
                }
            },
            Some(Token::LeftParenthesis) => {
                self.next();
                was_paren = true;
                Expression::Parenthesis(Box::new(self.expression(0)?))
            },
            _ => {self.parse_null_denotion()?},
        };
        
        if was_paren {
            assert_eq!(self.tokens.next(), Some(&Token::RightParenthesis));
        };

        while self.next_binds_tighter_than(rbp) {
            match self.tokens.peek() {
                Some(Token::RightParenthesis) => {continue},
                _ => {left = self.parse_left_denotion(left)?;}
            }
        }
        Ok(left)
    }

    fn next_binds_tighter_than(&mut self, right_binding_power: u32) -> bool {
        self.tokens.peek()
            .map_or(false, |t| { 
                t.left_binding_power() > right_binding_power
            }
        )
    }

    fn parse_null_denotion(&mut self) -> Result<Expression> {
        self.slices.next();
        self.tokens.next()
            .map_or(Err(anyhow!("incomplete expression".to_string())), |t| {
                t.null_denotion()
            }
        )
    }

    fn parse_left_denotion(&mut self, expr: Expression) -> Result<Expression> {
        self.slices.next();
        self.tokens.next()
            .map_or(Err(anyhow!("incomplete expression".to_string())), |t| {
                t.left_denotion(self, expr)
            }
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_parens_arithmatic() {
        let mut lexer = Token::lexer("pi*3+2");
        let (tokens, slices) = from_logos(&mut lexer);
        let mut parser = Parser::new(tokens.iter(), slices.iter());
        let g = Expression::Binary(
            Box::new(
                Expression::Binary(
                    Box::new(Expression::Numeric(3.141592653589793)), 
                    Token::Multiply, 
                    Box::new(Expression::Numeric(3.0))
                )
            ),
            Token::Plus, 
            Box::new(Expression::Numeric(2.0))
        );
        assert_eq!(parser.expression(0).unwrap(), g);
    }

    #[test]
    fn sin_pi_exp() {
        let mut lexer = Token::lexer("(sin(pi))^2");
        let (tokens, slices) = from_logos(&mut lexer);
        let mut parser = Parser::new(tokens.iter(), slices.iter());
        let g = Expression::Binary(
            Box::new(
                Expression::Parenthesis(
                    Box::new(   
                        Expression::LeftUnary(
                            "sin(".to_owned(), 
                            Box::new(Expression::Numeric(3.141592653589793))
                        )
                    ), 
                )
            ),
            Token::Exponentiate,
            Box::new(Expression::Numeric(2.0))
        );
        assert_eq!(parser.expression(0).unwrap(), g);
    }
}