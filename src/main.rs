use std::io::{stdin, stdout, Write};

mod parser;
mod data;
mod cli_interface;

fn main() {

    cli_interface::CliInterface::cli_interface_loop()

}
