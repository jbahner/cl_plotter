use crate::parser::tokenizer::Token;
use std::ops::{Div, Add, Sub};

pub struct Data<'a> {
    expr: &'a Token,
    min: f32,
    max: f32,
    n: usize,
    data: Vec<f32>,
}

impl Data<'_> {
    /// Instantiates a new Data representation
    pub fn new(expr: &Token, min: f32, max: f32, n: usize) -> Data {
        Data {
            expr,
            min,
            max,
            n,
            data: vec![],
        }
    }

    /// Evaluates the expression for a single value
    pub fn evaluate_single(self, val : f32) -> f32 {
        self.expr.clone().evaluate(val)
    }

    /// Calculates the interval size for the given amount of values in the given range
    fn interval_size(&self) -> f32 {
        (self.max - self.min).div(self.n as f32 - 1.0)
    }

    /// Calculates the index of a value in the data vector
    fn calculate_index(&self, val: f32) -> usize {
        let intervals = ((val - self.min).div(self.interval_size())).floor() as usize;
        if intervals > self.data.len() {
            self.data.len()
        } else {
            intervals
        }
    }

    /// Evaluate expression for n values in range [min, max]
    pub fn evaluate(&mut self) {
        // Clear old vector
        self.data.clear();

        let step = self.interval_size();
        for i in 0..self.n {
            self.data.push(self.expr.clone().evaluate(self.min + (i as f32 * step)));
        }
    }

    /// Calculates the minimum of the data in range [from, to)
    pub fn min(self, from: f32, to: f32) -> f32 {
        self.data[self.calculate_index(from)..self.calculate_index(to)].iter().cloned().fold(std::f32::MAX, f32::min)
    }

    /// Calculates the maximum of the data in range [from, to)
    pub fn max(self, from: f32, to: f32) -> f32 {
        self.data[self.calculate_index(from)..self.calculate_index(to)].iter().cloned().fold(std::f32::MIN, f32::max)
    }

    /// Calculates the differentiation of the expression, returning the values in a new vector, leaving the stored data unchanged
    pub fn differentiate(self) -> Vec<f32> {
        let interval = self.interval_size();
        let mut vector : Vec<f32> = vec![];
        for i in 0..self.data.len() {
            let x = self.min + (i as f32 * interval);
            vector.push(self.expr.clone().evaluate(x + interval).sub(self.expr.clone().evaluate(x - interval)).div(2.0 * interval))
        }
        vector
    }

    /// Calculates the integral of the expression for the given range
    pub fn integrate(self, from: f32, to: f32) -> f32 {
        let dx = self.interval_size() / 1000.0;
        let mut sum : f32 = 0.0;
        let num_steps = ((to - from) / dx) as usize;
        for i in 0..num_steps {
            sum += self.expr.clone().evaluate(from + (i as f32 * dx)) * dx;
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use crate::parser::tokenizer::Token::Digit;

    use super::*;

    #[test]
    fn evaluate_data_length() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(21, data.data.len());
    }

    #[test]
    fn evaluate_single() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        assert_eq!(47.0, data.evaluate_single(44.0).round());
    }

    #[test]
    fn interval_size() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        assert_eq!(1.0, data.interval_size());
    }

    #[test]
    fn calculate_index() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(10, data.calculate_index(0.0));
    }

    #[test]
    fn calculate_index_lower_bound() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(0, data.calculate_index(-10.0));
    }

    #[test]
    fn calculate_index_upper_bound() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(20, data.calculate_index(10.0));
    }

    #[test]
    fn calculate_index_out_of_bound() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(21, data.calculate_index(27.0));
    }

    #[test]
    fn calculate_min() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(-7, data.min(-10.0, 10.0).round() as i64);
    }

    #[test]
    fn calculate_max() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(13, data.max(-10.0, 11.0).round() as i64);
    }

    #[test]
    fn calculate_max_exclusive() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 21);

        data.evaluate();

        assert_eq!(12, data.max(-10.0, 10.0).round() as i64);
    }
}