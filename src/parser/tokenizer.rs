use std::fmt;
use Token::*;
extern crate colored;
use colored::*;

pub enum Token {
    Variable,
    Digit(i64),
    Addition(Box<Token>, Box<Token>),
    Subtraction(Box<Token>, Box<Token>),
    Multiplication(Box<Token>, Box<Token>),
    Division(Box<Token>, Box<Token>),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variable => {
                write!(f, "{}", "Variable".red())
            },
            Digit(val) => {
                write!(f, "{}", val.to_string().blue())
            },
            Addition(first, second) => {
                write!(f, "{} {}, {}{}", "Addition(".purple(), first, second, ")".purple())
            },
            Subtraction(first, second) => {
                write!(f, "{} {}, {}{}", "Subtraction(".cyan(), first, second, ")".cyan())
            },
            Multiplication(first, second) => {
                write!(f, "{} {}, {}{}", "Multiplication(".green(), first, second, ")".green())
            },
            Division(first, second) => {
                write!(f, "{} {}, {}{}", "Division(".yellow(), first, second, ")".yellow())
            },
        }
    }
}