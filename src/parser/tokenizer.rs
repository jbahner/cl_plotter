use std::fmt;
use Token::*;
extern crate colored;
use colored::*;

#[derive(PartialEq, Debug)]
/// Token enum containing all allowed input tokens
pub enum Token {
    Variable,
    Digit(f32),
    Addition(Box<Token>, Box<Token>),
    Subtraction(Box<Token>, Box<Token>),
    Multiplication(Box<Token>, Box<Token>),
    Division(Box<Token>, Box<Token>),
}

impl Token {
    /// Evaluates the token's expression with the given value replacing variables
    pub fn evaluate(self, var: f32) -> f32{
        match self {
            Variable => {var},
            Digit(d) => { d},
            Addition(first, second) => {
                first.evaluate(var) + second.evaluate(var)
            },
            Subtraction(first, second) => {
                first.evaluate(var) - second.evaluate(var)
            },
            Multiplication(first, second) => {
                first.evaluate(var) * second.evaluate(var)
            },
            Division(first, second) => {
                first.evaluate(var) / second.evaluate(var)
            },
        }
    }

    /// Checks if the token or any subtokens contain a variable
    pub fn contains_var(&self) -> bool{
        match self {
            Variable => {
                true
            },
            Addition(first, second) |
            Subtraction(first, second) |
            Multiplication(first, second) |
            Division(first, second) => {
                first.contains_var() || second.contains_var()
            }
            _ => {
                false
            }
        }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Variable => Variable,
            Digit(v) => Digit(v.clone()),
            Addition(first, second) => {
                Addition(first.clone(), second.clone())
            },
            Subtraction(first, second) => {
                Subtraction(first.clone(), second.clone())
            },
            Multiplication(first, second) => {
                Multiplication(first.clone(), second.clone())
            },
            Division(first, second) => {
                Division(first.clone(), second.clone())
            }
        }
    }
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
                write!(f, "{}{}, {}{}", "Addition(".purple(), first, second, ")".purple())
            },
            Subtraction(first, second) => {
                write!(f, "{}{}, {}{}", "Subtraction(".cyan(), first, second, ")".cyan())
            },
            Multiplication(first, second) => {
                write!(f, "{}{}, {}{}", "Multiplication(".green(), first, second, ")".green())
            },
            Division(first, second) => {
                write!(f, "{}{}, {}{}", "Division(".yellow(), first, second, ")".yellow())
            },
        }
    }
}