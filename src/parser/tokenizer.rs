use crate::parser::tokenizer::Token::Addition;

#[derive(Debug)]
pub enum Token {
    Variable,
    Digit(i64),
    Addition(Box<Token>, Box<Token>),
    Subtraction(Box<Token>, Box<Token>),
    Multiplication(Box<Token>, Box<Token>),
    Division(Box<Token>, Box<Token>),
}
