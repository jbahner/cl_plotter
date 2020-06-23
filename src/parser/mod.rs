use tokenizer::*;
use tokenizer::Token::*;

mod tokenizer;

pub struct Parser {
    pub stack: Vec<Token>,
}

impl Parser {

    pub fn clear(&mut self) {
        self.stack.clear();
    }

    pub fn display_expression(&mut self) {
        for t in self.stack.iter() {
            println!("{}", t);
        }
    }

    pub fn parse_expression(&mut self, expr: String) {
        println!("Input: {}", expr);
        let mut i = 0;
        while i < expr.len() {
            let char = expr.chars().nth(i).unwrap();
            match char {
                ' ' => {
                    i += 1;
                    continue;
                }
                '+' => {
                    let token = self.stack.pop().unwrap_or(Digit(0));
                    self.stack.push(Addition(Box::new(token), Box::new(Parser::get_next_token(&expr, i))));
                    i += 1;
                },
                '-' => {
                    let token = self.stack.pop().unwrap_or(Digit(0));
                    self.stack.push(Subtraction(Box::new(token), Box::new(Parser::get_next_token(&expr, i))));
                    i += 1;
                },
                '*' => {
                    let token = self.stack.pop().unwrap();
                    match token {
                        Digit(_) | Multiplication(_, _) | Division(_, _) => {
                            self.stack.push(Multiplication(Box::new(token), Box::new(Parser::get_next_token(&expr, i))));
                            i += 1;
                        },
                        Addition(first_val, second_val) => {
                            self.stack.push(Addition(first_val, Box::new(Multiplication(second_val, Box::new(Parser::get_next_token(&expr, i))))));
                            i += 1;
                        },
                        Subtraction(first_val, second_val) => {
                            self.stack.push(Subtraction(first_val, Box::new(Multiplication(second_val, Box::new(Parser::get_next_token(&expr, i))))));
                            i += 1;
                        },
                        _ => {}
                    }
                },
                '/' => {
                    let token = self.stack.pop().unwrap();
                    match token {
                        Digit(_) | Multiplication(_, _) | Division(_, _) => {
                            self.stack.push(Division(Box::new(token), Box::new(Parser::get_next_token(&expr, i))));
                            i += 1;
                        },
                        Addition(first_val, second_val) => {
                            self.stack.push(Addition(first_val, Box::new(Division(second_val, Box::new(Parser::get_next_token(&expr, i))))));
                            i += 1;
                        }
                        Subtraction(first_val, second_val) => {
                            self.stack.push(Subtraction(first_val, Box::new(Division(second_val, Box::new(Parser::get_next_token(&expr, i))))));
                            i += 1;
                        }
                        _ => {}
                    }
                }
                _ => {
                    if char.is_digit(10) {
                        self.stack.push(Digit(char.to_digit(10).unwrap() as i64));
                    } else {
                        self.stack.push(Variable);
                    }
                }
            }
            i += 1;
        }
    }

    fn get_next_token(expr : &str, idx: usize) -> Token {
        let char = expr.chars().nth(idx + 1).unwrap();
        return if char.is_digit(10) {
            Digit(char.to_digit(10).unwrap() as i64)
        } else {
            Variable
        }
    }
}

