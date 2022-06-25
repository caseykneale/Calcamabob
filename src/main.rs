use anyhow::{Result};
use logos::Logos;

mod lexer;
use crate::lexer::*;

mod interpreter;
use crate::interpreter::*;

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[clap(about, version, author)]
struct CliArgs {
    #[clap(short, long, value_parser)]
    expression: Option<String>,
    #[clap(short, long, value_parser)]
    file: Option<String>,
}

/// Lexes, parses, and interprets the numerical result(if successful)
/// of an input string.
/// 
/// # Arguments
///
/// * `corpus` - Some String containing text to be evaluated.
fn calculate(corpus: String) -> Result<f64> {
    println!("---\n");
    let mut lexer = Token::lexer(&corpus);
    let (tokens, slices) = from_logos(&mut lexer);
    println!("{:?}", tokens);
    let mut parser = lexer::Parser::new(tokens.iter(), slices.iter());
    let ast = parser.expression(0)?;
    println!("{:?}", ast);
    interpreter(ast)
}

fn main() {
    let args = CliArgs::parse();
    let result = match (args.expression, args.file) {
        (Some(expr), None) => calculate(expr),
        (None, Some(file_path)) => {
            let file_contents = std::fs::read_to_string(file_path)
                .unwrap();
            calculate(file_contents)
        },
        (Some(expr), Some(_)) => {
            println!("Both a file designation and an expression were given to Calcamabob. \
            Defaulting, to Expression, please correct your command line arguments.");
            calculate(expr)
        },
        (None, None) => {calculate("0.".to_owned())}
    };
    match result{
        Ok(value) => println!("{:?}", value),
        Err(error) => panic!("{}", error),
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
        assert_eq!(calculate("-(5+-4)".to_owned()).unwrap(), -1.);
        assert_eq!(calculate("-(5-4)".to_owned()).unwrap(), -1.);
        assert_eq!(calculate("5*10-4*3".to_owned()).unwrap(), 38.);
        assert_eq!(calculate("5*10-4*-3".to_owned()).unwrap(), 62.);
        assert_eq!(calculate(" 5  * 10- 4*-  3".to_owned()).unwrap(), 62.);
        assert_eq!(calculate("-cos(pi)".to_owned()).unwrap(), 1.);
        assert_eq!(calculate("log10(100.0)".to_owned()).unwrap(), 2.);
        assert_eq!(calculate("round(2/3)".to_owned()).unwrap(), 1.);
        assert_eq!(calculate("round(-1/4)".to_owned()).unwrap(), 0.);
        assert_eq!(calculate("-(tan(pi/4)*3)".to_owned()).unwrap().round(), -3.);
        assert_eq!(calculate("-3 + 4.2/333".to_owned()).unwrap(), -2.9873873873873873873873873873873873873873873873873873873873873873);
    }
}