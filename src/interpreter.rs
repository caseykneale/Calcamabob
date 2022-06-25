use crate::lexer::*;

pub fn interpreter(ast: Expression) -> f64 {
    match ast {
        Expression::Numeric(z) => z,
        Expression::LeftUnary(z, expr) => {
            match z.as_str() {
                "sin(" => interpreter(*expr).sin(),
                "cos(" => interpreter(*expr).cos(),
                "tan(" => interpreter(*expr).tan(),
                _ => panic!("I'm so scared"),
            }
        }
        Expression::Parenthesis(expr) => interpreter(*expr),
        Expression::Binary(left, operator, right) => { 
            match operator {
                Token::Divide => interpreter(*left) / interpreter(*right),
                Token::Multiply => interpreter(*left) * interpreter(*right),
                Token::Plus => interpreter(*left) + interpreter(*right),
                Token::Minus => interpreter(*left) - interpreter(*right),
                Token::Exponentiate => interpreter(*left).powf(interpreter(*right)),
                _ => panic!("I'm so scared"),
            }
        },
        // _ => {0.0}
    }
}
