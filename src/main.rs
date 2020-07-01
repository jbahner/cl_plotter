use std::io::{stdin, stdout, Write};
mod parser;
fn main() {
    let mut p = parser::Parser::new();
    loop {
        let mut input = String::new();
        print!("Type in an expression: ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Retard!");
        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }
        // TODO: Validate input format: only one variable allowed
        // TODO: here comes parsing of typed in command
        if input == String::from("exit") {
            println!("Bye, have a beautiful time!");
            std::process::exit(0);
        }
        p.parse_expression(input);
        p.display_expression();
        if p.contains_var() {
            print!("Evaluate with value: ");
            let _ = stdout().flush();
            input = String::new();
            stdin().read_line(&mut input).expect("Retard!");
            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }
            println!("{}", p.evaluate(input.parse().expect("Invalid number")));
        } else {
            p.evaluate(0.0);
        }
        p.clear();
    }
}
