use std::borrow::{Borrow, BorrowMut};

use tokenizer::*;
use tokenizer::Token::*;

pub mod tokenizer;

/// Used for parsing expressions to be evaluated
pub struct Parser {
    pub stack: Vec<Token>,
}

impl Parser {
    /// Instantiates parser with empty stack
    pub fn new() -> Parser {
        Parser {
            stack: vec![],
        }
    }

    /// Clears the stack
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    /// Displays the nested parsed expression in coloured output
    pub fn display_expression(&mut self) {
        for t in self.stack.iter() {
            println!("{}", t);
        }
    }

    /// Returns if the parsed expression contains a variable
    pub fn contains_var(&mut self) -> bool {
        for t in self.stack.iter() {
            if t.contains_var() {
                return true;
            }
        }
        return false;
    }

    /// Evaluates the parsed expression with the given value replacing variables
    pub fn evaluate(&mut self, var: f32) -> f32 {
        self.stack.pop().unwrap().evaluate(var)
    }

    /// Parses the given String to a Token expression
    fn parse(e: String) -> Token {
        let mut stack: Vec<Token> = vec![];
        let expr: String = e.chars().filter(|c| !c.is_whitespace()).collect();
        let mut i = 0;
        while i < expr.len() {
            // Get rid of whitespaces
            let char = expr.chars().nth(i).unwrap();
            i += 1;
            let mut next_consumed = 0;
            match char {
                '+' => {
                    let token = match stack.pop() {
                        None => Digit(0.0),
                        Some(t) => match t {
                            Bracket(v) => *v,
                            _ => t,
                        }
                    };

                    let (next_token, consumed) = Parser::get_next_token(&expr[i..]);
                    next_consumed += consumed;
                    stack.push(Addition(Box::new(token), Box::new(next_token)));
                }
                '-' => {
                    let token = match stack.pop() {
                        None => Digit(0.0),
                        Some(t) => match t {
                            Bracket(v) => *v,
                            _ => t,
                        }
                    };
                    let (next_token, consumed) = Parser::get_next_token(&expr[i..]);
                    next_consumed += consumed;
                    stack.push(Subtraction(Box::new(token), Box::new(next_token)));
                }
                '*' => {
                    let token = stack.pop().unwrap();
                    let (next_token, consumed) = Parser::get_next_token(&expr[i..]);
                    next_consumed += consumed;
                    match token {
                        Digit(_) | Multiplication(_, _) | Division(_, _) | Variable => {
                            stack.push(Multiplication(Box::new(token), Box::new(next_token)));
                        }
                        Bracket(t) => {
                            stack.push(Multiplication(t, Box::new(next_token)));
                        }
                        Addition(first_val, second_val) => {
                            stack.push(Addition(first_val, Box::new(Multiplication(second_val, Box::new(next_token)))));
                        }
                        Subtraction(first_val, second_val) => {
                            stack.push(Subtraction(first_val, Box::new(Multiplication(second_val, Box::new(next_token)))));
                        }
                    }
                }
                '/' => {
                    let token = stack.pop().unwrap();
                    let (next_token, consumed) = Parser::get_next_token(&expr[i..]);
                    next_consumed += consumed;
                    match token {
                        Digit(_) | Multiplication(_, _) | Division(_, _) | Variable => {
                            stack.push(Division(Box::new(token), Box::new(next_token)));
                        }
                        Bracket(t) => {
                            stack.push(Division(t, Box::new(next_token)));
                        }
                        Addition(first_val, second_val) => {
                            stack.push(Addition(first_val, Box::new(Division(second_val, Box::new(next_token)))));
                        }
                        Subtraction(first_val, second_val) => {
                            stack.push(Subtraction(first_val, Box::new(Division(second_val, Box::new(next_token)))));
                        }
                    }
                }
                _ => {
                    let (next_token, consumed) = Parser::get_next_token(&expr[i - 1..]);
                    next_consumed += consumed - 1;
                    stack.push(next_token);
                }
            }
            i += next_consumed;
        }
        return match stack.pop() {
            Some(Bracket(v)) => *v,
            Some(t) => t,
            None => panic!("Could not parse")
        };
    }

    /// Parses the given expression for later evaluation
    pub fn parse_expression(&mut self, e: String) {
        self.stack.push(Self::parse(e));
    }

    /// Retrieves the first token and the index of it in the expr String
    fn get_next_token(expr: &str) -> (Token, usize) {
        let mut i = 1;
        let char = expr.chars().nth(0).unwrap();
        return if char.is_digit(10) {
            let mut float = char.to_string();
            while i < expr.len() {
                let next_token = expr.chars().nth(i).unwrap();
                if next_token == '.' || next_token.is_digit(10) {
                    float.push(next_token);
                    i += 1;
                } else {
                    break;
                }
            }
            (Digit(float.parse().unwrap()), i)
        } else if char == '(' {
            let mut idx = expr.find(')').unwrap() + 1;

            while Self::count_occurrences('(', &expr[0..idx]) != Self::count_occurrences(')', &expr[0..idx]) {
                idx += expr[idx..].find(')').unwrap() + 1;
            }

            let t = Self::parse(String::from(&expr[1..idx - 1]));
            (t, idx)
        } else {
            
            (Variable, i)
        };
    }

    /// Counts occurrences of character c in string s
    pub fn count_occurrences(c: char, s: &str) -> usize {
        let mut count = 0;
        for e in s.chars() {
            if e == c {
                count += 1;
            }
        }
        count
    }
}

impl Clone for Parser {
    fn clone(&self) -> Self {
        Parser {
            stack: self.stack.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn next_token_digit() {
        let expr = "1+2";

        let (token, length) = Parser::get_next_token(expr);

        assert_eq!(token, Digit(1.0));
        assert_eq!(length, 1);
    }

    #[test]
    fn next_token_variable() {
        let expr = "1+x";

        let (token, length) = Parser::get_next_token(&expr[1..]);

        assert_eq!(token, Variable);
        assert_eq!(length, 1);
    }

    #[test]
    fn parse_plus() {
        let mut parser = Parser::new();
        let expr = String::from("1+2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Addition(Box::new(Digit(1.0)), Box::new(Digit(2.0))));
    }

    #[test]
    fn parse_minus() {
        let mut parser = Parser::new();
        let expr = String::from("1-2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Subtraction(Box::new(Digit(1.0)), Box::new(Digit(2.0))));
    }

    #[test]
    fn parse_mult() {
        let mut parser = Parser::new();
        let expr = String::from("1*2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Multiplication(Box::new(Digit(1.0)), Box::new(Digit(2.0))));
    }

    #[test]
    fn parse_division() {
        let mut parser = Parser::new();
        let expr = String::from("1/2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Division(Box::new(Digit(1.0)), Box::new(Digit(2.0))));
    }

    #[test]
    fn parse_concatenation() {
        let mut parser = Parser::new();
        let expr = String::from("1+2-3*4/5");

        parser.parse_expression(expr);

        // Don't judge pls
        assert_eq!(parser.stack.pop().unwrap(), Subtraction(Box::new(Addition(Box::new(Digit(1.0)), Box::new(Digit(2.0)))), Box::new(Division(Box::new(Multiplication(Box::new(Digit(3.0)), Box::new(Digit(4.0)))), Box::new(Digit(5.0))))));
    }

    #[test]
    fn evaluate_plus() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10.0)), Box::new(Variable))]
        };

        assert_eq!(15.0, parser.evaluate(5.0));
    }

    #[test]
    fn evaluate_minus() {
        let mut parser = Parser {
            stack: vec![Subtraction(Box::new(Digit(10.0)), Box::new(Variable))]
        };

        assert_eq!(5.0, parser.evaluate(5.0));
    }

    #[test]
    fn evaluate_mult() {
        let mut parser = Parser {
            stack: vec![Multiplication(Box::new(Digit(10.0)), Box::new(Variable))]
        };

        assert_eq!(50.0, parser.evaluate(5.0));
    }

    #[test]
    fn evaluate_division() {
        let mut parser = Parser {
            stack: vec![Division(Box::new(Digit(10.0)), Box::new(Variable))]
        };

        assert_eq!(2.0, parser.evaluate(5.0));
    }

    #[test]
    fn evaluate_concatenation() {
        // x + 2 - 3 * 4 / 5
        let mut parser = Parser {
            stack: vec![Subtraction(Box::new(Addition(Box::new(Variable), Box::new(Digit(2.0)))), Box::new(Division(Box::new(Multiplication(Box::new(Digit(3.0)), Box::new(Digit(4.0)))), Box::new(Digit(5.0)))))]
        };

        assert_eq!(4.6, parser.evaluate(5.0));
    }

    #[test]
    fn contains_var_false() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10.0)), Box::new(Digit(2.0)))]
        };

        assert!(!parser.contains_var());
    }

    #[test]
    fn contains_var_true() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10.0)), Box::new(Variable))]
        };

        assert!(parser.contains_var());
    }

    #[test]
    fn contains_var_nested() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10.0)), Box::new(Multiplication(Box::new(Variable), Box::new(Digit(2.0)))))]
        };

        assert!(parser.contains_var());
    }

    #[test]
    fn clear_stack() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10.0)), Box::new(Variable))]
        };

        parser.clear();

        assert!(parser.stack.is_empty());
    }
}
