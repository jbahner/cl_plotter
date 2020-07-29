use std::io::{stdin, stdout, Write};

mod parser;
mod data;
mod cli_interface;

fn main() {
    let mut p = parser::Parser::new();




        // print!("Tell me what to do: ");
        // let _ = stdout().flush();
        // stdin().read_line(&mut input).expect("Retard!");
        cli_interface::CliInterface::cli_interface_loop()
        //
        // if let Some('\n') = input.chars().next_back() {
        //     input.pop();
        // }
        // if let Some('\r') = input.chars().next_back() {
        //     input.pop();
        // }
        // TODO: here comes parsing of typed in command
        // println!("Input: {}", input);
}
