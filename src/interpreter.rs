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
                "radian(" => Ok(interpreter(*expr)?.to_radians()),
                "degrees(" => Ok(interpreter(*expr)?.to_degrees()),
                "round(" => Ok(interpreter(*expr)?.round()),
                "trunc(" => Ok(interpreter(*expr)?.trunc()),
                "abs(" => Ok(interpreter(*expr)?.abs()),
                "ln(" => Ok(interpreter(*expr)?.ln()),
                "log10(" => Ok(interpreter(*expr)?.log10()),
                "log2(" => Ok(interpreter(*expr)?.log2()), 
                "sqrt(" => Ok(interpreter(*expr)?.sqrt()),
                "asin(" => Ok(interpreter(*expr)?.asin()),
                "acos(" => Ok(interpreter(*expr)?.acos()),
                "atan(" => Ok(interpreter(*expr)?.atan()),
                "sin(" => Ok(interpreter(*expr)?.sin()),
                "cos(" => Ok(interpreter(*expr)?.cos()),
                "tan(" => Ok(interpreter(*expr)?.tan()),
                "sinh(" => Ok(interpreter(*expr)?.sinh()),
                "cosh(" => Ok(interpreter(*expr)?.cosh()),
                "tanh(" => Ok(interpreter(*expr)?.tanh()),
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
