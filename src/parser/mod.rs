use tokenizer::*;
use tokenizer::Token::*;

mod tokenizer;

pub struct Parser {
    pub stack: Vec<Token>,
}

impl Parser {   

    pub fn parse_expression(&mut self, expr: String) {
        println!("Input: {}", expr);
        let mut i = 0;
        while i < expr.len() {
            let char = expr.chars().nth(i).unwrap();
            println!("char is {} at {}", char, i);
            match char {
                ' ' => {
                    continue;
                }
                '+' => {
                    println!("Pushing +");
                    let token = self.stack.pop().unwrap_or(Digit(0));
                    self.stack.push(Addition(Box::new(token), Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))));
                    i += 1;
                },
                '-' => {
                    println!("Pushing -");
                    let token = self.stack.pop().unwrap_or(Digit(0));
                    self.stack.push(Subtraction(Box::new(token), Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))));
                    i += 1;
                },
                '*' => {
                    let token = self.stack.pop().unwrap();
                    match token {
                        Digit(_) | Multiplication(_, _) | Division(_, _) => {
                            self.stack.push(Multiplication(Box::new(token), Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))));
                            i += 1;
                        },
                        Addition(firstVal, secondVal) => {
                            self.stack.push(Addition(firstVal, Box::new(Multiplication(secondVal, Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))))));
                            i += 1;
                        },
                        Subtraction(firstVal, secondVal) => {
                            self.stack.push(Subtraction(firstVal, Box::new(Multiplication(secondVal, Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))))));
                            i += 1;
                        },
                        _ => {}
                    }
                },
                '/' => {
                    let token = self.stack.pop().unwrap();
                    match token {
                        Digit(_) | Multiplication(_, _) | Division(_, _) => {
                            self.stack.push(Division(Box::new(token), Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))));
                            i += 1;
                        },
                        Addition(firstVal, secondVal) => {
                            self.stack.push(Addition(firstVal, Box::new(Division(secondVal, Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))))));
                            i += 1;
                        },
                        Subtraction(firstVal, secondVal) => {
                            self.stack.push(Subtraction(firstVal, Box::new(Division(secondVal, Box::new(Digit(expr.chars().nth(i + 1).unwrap().to_digit(10).unwrap() as i64))))));
                            i += 1;
                        },
                        _ => {}
                    }
                }
                _ => {
                    if char.is_digit(10) {
                        println!("Pushing digit {}", char);
                        self.stack.push(Digit(char.to_digit(10).unwrap() as i64));
                    } else {
                        panic!("Unkown character!");
                    }
                }
            }
            i += 1;
        }
        for t in self.stack.iter() {
            println!("{:?}", t);
        }
    }
}

