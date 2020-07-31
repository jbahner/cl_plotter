use std::borrow::{Borrow, BorrowMut};
use std::fmt::format;
use std::io::{stdin, stdout, Write};
use std::iter;
use std::ops::Rem;
use std::ptr::null;
use std::thread::current;

use terminal_size::{Height, terminal_size, Width};

use crate::data::Data;
use crate::parser::Parser;
use crate::parser::tokenizer::Token;

use super::data;
use super::parser;

pub struct CliInterface {
    pub calculation_density: i64,

}

static mut current_state: &str = "start";
static mut X_MIN: i64 = 0;
static mut X_MAX: i64 = 5;
static mut Y_MIN: i64 = 0;
static mut Y_MAX: i64 = 10;
static mut saved_functions: Vec<String> = Vec::new();

impl CliInterface {
    const UI_LEFT_MARGIN: u16 = 8;
    const UI_RIGHT_MARGIN: u16 = 6;

    // Header Space should be an even number
    const UI_HEADER_SPACE: u16 = 6;
    const UI_FOOTER_SPACE: u16 = 6;

    const MINIMUM_WIDTH: u16 = 85;

    const PLOT_GRAPH_CHARACTER: char = '*';


    pub fn cli_interface_loop() {
        let mut p = parser::Parser::new();

        // TODO change this back to "start"
        Self::draw_screen(&"start".to_string());

        loop {
            let mut input = String::new();

            let _ = stdout().flush();

            println!("Type commands below!\n");
            // TODO please dont say retard
            stdin().read_line(&mut input).expect("Retard!");

            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }

            // TODO: Validate input format: only one variable allowed
            // TODO: here comes parsing of typed in command

            p = Self::process_input(input, p.clone());

            p.clear();
        }
    }

    fn process_input(input: String, mut parser: Parser) -> Parser {
        let input_arguments: Vec<&str> = input.split(" ").collect();

        match &input_arguments.get(0).unwrap()[..] {
            "input" => {
                let given_function = input_arguments[1..].join(" ");
                unsafe { saved_functions.push(given_function.clone()) }
                println!("Insertion successful!\n\n");
                // parser.parse_expression(given_function);
                // parser.display_expression();
            }
            "ls" => unsafe {
                println!("All saved Functions:");
                for x in 0..saved_functions.len() {
                    println!(" {}:  {}", x, saved_functions.get(x).unwrap());
                }
                print!("\n");

            }
            "rm" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser
                }
                if input_arguments[1].parse::<u16>().unwrap() < 0 || input_arguments[1].parse::<u16>().unwrap() >= saved_functions.len() as u16 {
                    println!("Deletion index is wrong!");
                    return parser
                }
                let remove_index = input_arguments[1].parse::<u16>().unwrap() as usize;
                saved_functions.remove(remove_index);

                println!("Deleted function at index: {}", remove_index);

                print!("\n");

            }
            "plot" => unsafe {
                current_state = "plot";
                Self::draw_screen("plot")
            }
            "redraw" => unsafe {
                Self::draw_screen(current_state)
            }
            "help" => unsafe {
                current_state = "start";
                Self::draw_screen(current_state)
            }
            "exit" => {
                println!("Bye, have a beautiful time!");
                std::process::exit(0);
            }
            _ => println!("Unknown command, type \"help\" for a List of available commands!")
        }

        parser
    }


    fn draw_screen(s: &str) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            if w < Self::MINIMUM_WIDTH {
                //TODO check height too
                panic!("Terminal window too small please make it wider!")
            }

            match &s[..] {
                "start" => Self::draw_start_screen(w, h),
                "plot" => Self::draw_plot(w, h),

                _ => println!("Unknown draw screen command!")
            }
        }
    }


    fn draw_start_screen(width: u16, height: u16) {
        let mut screen = String::new();
        let mut printed_lines = 0;

        // Draw Header Line
        screen += &iter::repeat("_").take(width as usize).collect::<String>();
        screen += &String::from("\n");

        // Draw Header Space above Headline
        for _ in 1..(CliInterface::UI_HEADER_SPACE / 2) {
            screen += &String::from("\n");
        }

        // Draw Headline
        screen += &Self::generate_centered_text_string(width, &String::from("Welcome to cl_plotter!"));

        // Draw Header Space below Headline
        for _ in 0..(CliInterface::UI_HEADER_SPACE / 2) {
            screen += &String::from("\n");
        }


        // Draw Main Body                                    "+ 2" is a magic number to make the UI fit the terminal
        let main_body_height = height - (CliInterface::UI_HEADER_SPACE + CliInterface::UI_FOOTER_SPACE + 2);
        // TODO do this more often
        let left_padding = iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>();
        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can input your linear functions here:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"input <your function>\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can list your linear functions:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"ls\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can remove a function from the list:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"rm <function-list-index>\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can redraw the current Ui:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"redraw\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can redraw the current Ui:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"redraw\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can plot the current function:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"plot\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Or exit by typing:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"exit\"\n\n"));
        printed_lines += 3;

        for _ in 0..(main_body_height - printed_lines) {
            screen += &String::from("\n");
        }


        // Draw Footer
        for _ in 0..(CliInterface::UI_FOOTER_SPACE - 4) {
            screen += &String::from("\n");
        }

        print!("{}", screen);
        println!("{}\n", iter::repeat("_").take(width as usize).collect::<String>());
    }


    /// Return String containing message centered by white spaces
    /// Panics when Message is longer than the width of the terminal
    fn generate_centered_text_string(width: u16, message: &str) -> String {
        if width < message.len() as u16 {
            panic!("Centered context message is too wide to be displayed");
        }

        let mut str = String::new();

        for _ in 0..((width - message.len() as u16) / 2) {
            str += &String::from(" ");
        }

        str += message;

        str += &String::from("\n");

        return str;
    }


    fn draw_plot(width: u16, height: u16) {
        let mut screen = String::new();

        let graph_width = width - (Self::UI_LEFT_MARGIN + Self::UI_RIGHT_MARGIN + 1);
        let graph_height = height - (Self::UI_HEADER_SPACE + Self::UI_FOOTER_SPACE + 3);

        let function_vec: Vec<f32>;
        // let function_vec2: Vec<f32>;

        let mut data: Data;
        unsafe {
            let expr = Token::Multiplication(Box::new(Token::Variable), Box::new(Token::Variable));
            data = data::Data::new(&expr, X_MIN as f32, X_MAX as f32, graph_width as usize);
            data.evaluate();
            function_vec = data.data.clone();
            // function_vec2 = data.differentiate();
        }

        let mut data_matrix = Self::plot_values_into_matrix(vec![vec![" "; graph_width as usize]; graph_height as usize],
                                                        function_vec,
                                                        height);

        // data_matrix = Self::plot_values_into_matrix(data_matrix,
        //                                                     function_vec2,
        //                                                     height);


        for current_height in 0..height - 3 {
            if current_height < Self::UI_HEADER_SPACE {
                screen += &Self::draw_header(current_height, width)
            } else if current_height >= height - Self::UI_FOOTER_SPACE {
                screen += &Self::draw_footer()
            } else {
                screen += &Self::draw_graph_line(width, height, current_height, data_matrix.clone());
            }
        }
        print!("{}", screen);
    }


    fn plot_values_into_matrix(mut data_matrix: Vec<Vec<&str>>, function_vec: Vec<f32>, height: u16) -> Vec<Vec<&str>> {
        let graph_height = height - (Self::UI_HEADER_SPACE + Self::UI_FOOTER_SPACE + 3);

        let mut graph_height_per_pixel: f64;
        unsafe { graph_height_per_pixel = (Y_MIN + Y_MAX) as f64 / graph_height as f64; };

        for func_value_index in 0..function_vec.len() {
            let func_value = function_vec.get(func_value_index).unwrap();

            for i in 0..graph_height {
                unsafe {
                    let mut current_y = Y_MIN as f64 + (graph_height_per_pixel * i as f64);
                    let next_y = Y_MIN as f64 + (graph_height_per_pixel * (i + 1) as f64);

                    if current_y < *func_value as f64 && *func_value as f64 <= next_y {
                        data_matrix[i as usize][func_value_index] = "*";
                    }
                }
            }
        }
        data_matrix
    }


    fn draw_graph_line(width: u16, height: u16, current_row: u16, data_matrix: Vec<Vec<&str>>) -> String {
        let mut screen = String::new();

        let graph_height = height - (Self::UI_HEADER_SPACE + Self::UI_FOOTER_SPACE + 3);


        if graph_height as i16 - (current_row as i16 - Self::UI_HEADER_SPACE as i16) < 0 {
            return "".to_string();
        }

        let current_height_relative_to_graph = graph_height - (current_row - Self::UI_HEADER_SPACE);
        // println!("Current Position: {}", current_height_relative_to_graph);


        if current_row == height - (Self::UI_FOOTER_SPACE + 3) {
            screen += &Self::draw_x_axis(width as i32)
        } else if current_row == height - (Self::UI_FOOTER_SPACE + 2) {
            screen += &Self::draw_x_axis_legend(width as i32)
        } else if current_row == height - (Self::UI_FOOTER_SPACE + 1) {
            screen += "X-Axis-Legend\n";
            // &Self::draw_x_axis_legend_numbers(width as i32)
        } else {
            // Y Axis
            screen += &(iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>() + "|");

            for plot_string in data_matrix.get((current_height_relative_to_graph - 1) as usize).unwrap() {
                screen += plot_string;
            }

            screen += "\n";
        }
        screen
    }

    fn draw_header(current_height: u16, width: u16) -> String {
        if current_height == 2 {
            return format!("{}", Self::generate_centered_text_string(width, "Your Plot"));
        }
        format!("{}", "\n")
    }

    fn draw_footer() -> String {
        String::from("f\n")
    }

    fn draw_x_axis(width: i32) -> String {
        format!("{}{}{}",
                iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>(),
                "|",
                iter::repeat("_").take(((width - 1) - Self::UI_LEFT_MARGIN as i32 - Self::UI_RIGHT_MARGIN as i32) as usize).collect::<String>() + "\n")

    }

    fn draw_x_axis_legend(width: i32) -> String {
        format!("{}{}",
                iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>(),
                iter::repeat("    |").take(((width - Self::UI_LEFT_MARGIN as i32 - Self::UI_RIGHT_MARGIN as i32) / 5) as usize).collect::<String>() + "\n")
    }

    fn draw_x_axis_legend_numbers(width: i32) -> String {
        let mut screen = String::new();

        screen += &iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>();

        for x in 2..width / 4 {
            if x < 10 {
                screen += &format!("{}{}",
                                   "   ".to_string(),
                                   x);
            } else {
                screen += &format!("{}{}",
                                   "  ".to_string(),
                                   x);
            }
        }
        screen += "\n";
        screen
    }
}