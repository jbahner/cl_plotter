use crate::parser::tokenizer::Token;

pub struct Data<'a> {
    expr: &'a Token,
    data: Vec<f32>,
}

impl Data<'_> {

    pub fn new(token: &Token) -> Data {
        Data {
            expr: token,
            // TODO: Range given as parameter
            data: vec![0.0; 25],
        }
    }

    pub fn evaluate(&mut self) {
        for i in 0..self.data.len()-1 {
            self.data[i] = self.expr.clone().evaluate(i as f32);
        }
    }
}