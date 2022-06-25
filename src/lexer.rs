use std::{slice::Iter, iter::Peekable};
use anyhow::{Result, anyhow};
use logos::Logos;

/// Returns a 2-tuple of the tokens, and Strings in a body of text as
/// they can be interpretted by Calcamabob.
/// 
/// # Arguments
///
/// * `lexer` - a mutable reference to a Logos Lexer
pub fn from_logos(lexer: &mut logos::Lexer<Token>) -> (Vec<Token>, Vec<String>) {
    let mut lexer_tokens:Vec<Token> = Vec::new();
    let mut lexer_slices:Vec<String> = Vec::new();
    let mut last_valid_token: Token = Token::Error;
    // let mut last_valid_slice: String = "".to_owned();
    loop {
        match lexer.next(){
            Some(Token::Error) => {continue;},
            Some(x) => {
                match (&last_valid_token, &x ) {
                    (Token::Numeric(_),Token::Numeric(b)) => {
                        if *b < 0.0 {
                            lexer_tokens.push(Token::Plus);
                            lexer_slices.push("+".to_owned());
                        }
                    },
                    _ => {}
                }
                lexer_tokens.push(x.clone());
                lexer_slices.push(lexer.slice().to_owned());
                last_valid_token = x;
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
    #[token("+", priority = 2)]
    Plus,
    #[token("-", priority = 2)]
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
    #[regex(r"([a-zA-Z]+)?([0-9]+)\(", priority = 10)] //Allow for log10()
    FunctionCall,

    #[regex("[-]?([0-9]*[.])?[0-9]+", |lex| lex.slice().parse::<f64>().unwrap(), priority = 1)]
    Numeric(f64),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip, priority = 1)]
    Error,
}

impl Token {
    /// Returns true if a token is an infix operator, false
    /// if not.
    /// 
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

    /// In the Pratt sense, calculates the left binding power 
    /// of this token.
    /// 
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
    
    /// Returns the expression (if it is valid) from a null 
    /// denotion token.
    /// 
    fn null_denotion(&self) -> Result<Expression> {
        match *self {
            Token::Numeric(f) => Ok(Expression::Numeric(f)),
            Token::PI(f) => Ok(Expression::Numeric(f)),
            Token::EulersConstant(f) => Ok(Expression::Numeric(f)),
            _ => Err(anyhow!("expecting literal"))
	    }
    }

    /// Returns the expression (if it is valid) from a left 
    /// denotion token.
    /// 
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

    /// Create a new Parser from streams of Tokens, and Strings.
    /// 
    /// # Arguments
    ///
    /// * `tokens` - an iterable token stream, probably from Logos.
    /// * `slices` - an iterable stream of token strings, probably from Logos.
    pub fn new(tokens: Iter<'a, Token>, slices: Iter<'a, String>) -> Self {
        Parser{ tokens: tokens.peekable(), slices: slices.peekable() }
    }

    /// Pratt parser
    /// 
    /// # Arguments
    ///
    /// * `rbp` - right binding power of the expression. Initialize with 0.
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
    fn no_parens_arithmetic() {
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