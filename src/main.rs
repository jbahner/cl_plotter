use std::io::{stdin, stdout, Write};

fn main() {
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
        // TODO: here comes parsing of typed in command
        println!("Input: {}", input);
    }
}
