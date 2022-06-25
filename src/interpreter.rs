use crate::lexer::*;
use anyhow::{Result, anyhow};

/// Returns the numerical result from the parsed AST.
/// 
/// # Arguments
///
/// * `ast` - An abstract binary syntax tree generated from the parser.
/// 
/// ToDo: Consider returning Result<f64>.
pub fn interpreter(ast: Expression) -> Result<f64> {
    match ast {
        Expression::Numeric(z) => Ok(z),
        Expression::LeftUnary(z, expr) => {
            match z.as_str() {
                "sqrt(" => Ok(interpreter(*expr)?.sqrt()),
                "asin(" => Ok(interpreter(*expr)?.asin()),
                "acos(" => Ok(interpreter(*expr)?.acos()),
                "atan(" => Ok(interpreter(*expr)?.atan()),
                "sin(" => Ok(interpreter(*expr)?.sin()),
                "cos(" => Ok(interpreter(*expr)?.cos()),
                "tan(" => Ok(interpreter(*expr)?.tan()),
                _ => Err(anyhow!("Prefix unary operator {:?} not avaialble.", z)),
            }
        }
        Expression::Parenthesis(expr) => Ok(interpreter(*expr)?),
        Expression::Binary(left, operator, right) => { 
            match operator {
                Token::Divide => Ok(interpreter(*left)? / interpreter(*right)?),
                Token::Multiply => Ok(interpreter(*left)? * interpreter(*right)?),
                Token::Plus => Ok(interpreter(*left)? + interpreter(*right)?),
                Token::Minus => Ok(interpreter(*left)? - interpreter(*right)?),
                Token::Exponentiate => Ok(interpreter(*left)?.powf(interpreter(*right)?)),
                _ => Err(anyhow!("Prefix infix operator {:?} not avaialble.", operator)),
            }
        },
    }
}
