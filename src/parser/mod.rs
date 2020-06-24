use tokenizer::*;
use tokenizer::Token::*;

mod tokenizer;

pub struct Parser {
    stack: Vec<Token>,
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

    /// Parses the given expression for later evaluation
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

#[cfg(test)]
mod test {
    use super::*;
    // use super::tokenizer::Token::*;
    #[test]
    fn next_token_digit() {
        let expr = "1+2";

        assert_eq!(Parser::get_next_token(expr, 1), Digit(2));
    }

    #[test]
    fn next_token_variable() {
        let expr = "1+x";

        assert_eq!(Parser::get_next_token(expr, 1), Variable);
    }

    #[test]
    fn parse_plus() {
        let mut parser = Parser::new();
        let expr = String::from("1+2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Addition(Box::new(Digit(1)), Box::new(Digit(2))));
    }

    #[test]
    fn parse_minus() {
        let mut parser = Parser::new();
        let expr = String::from("1-2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Subtraction(Box::new(Digit(1)), Box::new(Digit(2))));
    }

    #[test]
    fn parse_mult() {
        let mut parser = Parser::new();
        let expr = String::from("1*2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Multiplication(Box::new(Digit(1)), Box::new(Digit(2))));
    }

    #[test]
    fn parse_division() {
        let mut parser = Parser::new();
        let expr = String::from("1/2");

        parser.parse_expression(expr);

        assert_eq!(parser.stack.pop().unwrap(), Division(Box::new(Digit(1)), Box::new(Digit(2))));
    }

    #[test]
    fn parse_concatenation() {
        let mut parser = Parser::new();
        let expr = String::from("1+2-3*4/5");

        parser.parse_expression(expr);

        // Don't judge pls
        assert_eq!(parser.stack.pop().unwrap(), Subtraction(Box::new(Addition(Box::new(Digit(1)), Box::new(Digit(2)))), Box::new(Division(Box::new(Multiplication(Box::new(Digit(3)), Box::new(Digit(4)))), Box::new(Digit(5))))));
    }

    #[test]
    fn evaluate_plus() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10)), Box::new(Variable))]
        };

        assert_eq!(15, parser.evaluate(5));
    }

    #[test]
    fn evaluate_minus() {
        let mut parser = Parser {
            stack: vec![Subtraction(Box::new(Digit(10)), Box::new(Variable))]
        };

        assert_eq!(5, parser.evaluate(5));
    }

    #[test]
    fn evaluate_mult() {
        let mut parser = Parser {
            stack: vec![Multiplication(Box::new(Digit(10)), Box::new(Variable))]
        };

        assert_eq!(50, parser.evaluate(5));
    }

    #[test]
    fn evaluate_division() {
        let mut parser = Parser {
            stack: vec![Division(Box::new(Digit(10)), Box::new(Variable))]
        };

        assert_eq!(2, parser.evaluate(5));
    }

    #[test]
    fn evaluate_concatenation() {
        let mut parser = Parser {
            stack: vec![Subtraction(Box::new(Addition(Box::new(Variable), Box::new(Digit(2)))), Box::new(Division(Box::new(Multiplication(Box::new(Digit(3)), Box::new(Digit(4)))), Box::new(Digit(5)))))]
        };

        assert_eq!(5, parser.evaluate(5));
    }

    #[test]
    fn contains_var_false() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10)), Box::new(Digit(2)))]
        };

        assert!(!parser.contains_var());
    }

    #[test]
    fn contains_var_true() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10)), Box::new(Variable))]
        };

        assert!(parser.contains_var());
    }

    #[test]
    fn contains_var_nested() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10)), Box::new(Multiplication(Box::new(Variable), Box::new(Digit(2)))))]
        };

        assert!(parser.contains_var());
    }

    #[test]
    fn clear_stack() {
        let mut parser = Parser {
            stack: vec![Addition(Box::new(Digit(10)), Box::new(Variable))]
        };

        parser.clear();

        assert!(parser.stack.is_empty());
    }
}
