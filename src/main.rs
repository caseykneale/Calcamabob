use anyhow::{Result};
use logos::Logos;

mod lexer;
use crate::lexer::*;

mod interpreter;
use crate::interpreter::*;

use clap::Parser;

#[derive(clap::Parser, Debug)]
struct CliArgs {
    #[clap(short, long, value_parser)]
    eqn: String,
}

/// Lexes, parses, and interprets the numerical result(if successful)
/// of an input string.
/// 
/// # Arguments
///
/// * `corpus` - Some String containing text to be evaluated.
fn calculate(corpus: String) -> Result<f64> {
    let mut lexer = Token::lexer(&corpus);
    let (tokens, slices) = from_logos(&mut lexer);
    let mut parser = lexer::Parser::new(tokens.iter(), slices.iter());
    let ast = parser.expression(0)?;
    Ok(interpreter(ast))
}

fn main() {
    let args = CliArgs::parse();
    match calculate(args.eqn){
        Ok(value) => println!("{:?}", value),
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interpretted_results() {
        assert_eq!(calculate("(2+5*2)^2".to_owned()).unwrap(), 144.0);
        assert_eq!(calculate("2+5*2^2".to_owned()).unwrap(), 22.0);
        assert_eq!(calculate("cos(pi)".to_owned()).unwrap(), -1.);
        assert_eq!(calculate("sin(0)".to_owned()).unwrap(), 0.);
    }
}