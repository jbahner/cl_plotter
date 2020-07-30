use crate::parser::tokenizer::Token;

pub struct Data<'a> {
    expr: &'a Token,
    min: f64,
    max: f64,
    n: usize,
    data: Vec<f32>,
}

impl Data<'_> {

    /// Instantiates a new Data representation
    pub fn new(expr: &Token, min: f64, max: f64, n: usize) -> Data {
        Data {
            expr,
            min,
            max,
            n,
            data: vec![],
        }
    }

    /// Evaluate expression for n values in range defined by min and max
    pub fn evaluate(&mut self) {
        // Clear old vector
        self.data.clear();

        let step = (self.max - self.min) / self.n as f64;
        let mut i = self.min;
        while i < self.max {
            self.data.push(self.expr.clone().evaluate(i as f32));
            println!("x: {}, y: {}", i, self.data.last().unwrap());
            i += step;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::tokenizer::Token::Digit;

    #[test]
    fn evaluate_data_length() {
        let expr = Token::Addition(Box::new(Token::Variable), Box::new(Digit(3.0)));
        let mut data = Data::new(&expr, -10.0, 10.0, 20);

        data.evaluate();

        assert_eq!(20, data.data.len());
    }
}