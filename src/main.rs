use std::io::{stdin, stdout, Write};
mod parser;
fn main() {
    let mut p = parser::Parser {
        stack: vec![],
    };
    loop {
        let mut input = String::new();
        print!("Tell me what to do: ");
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
        p.clear();
    }
}
