use std::borrow::{Borrow, BorrowMut};
use std::fmt::format;
use std::io::{stdin, stdout, Write};
use std::iter;
use std::ops::Rem;
use std::ptr::null;
use std::thread::current;

use terminal_size::{Height, terminal_size, Width};

extern crate regex;
use regex::Regex;
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
static mut saved_functions: Vec<(String, bool)> = Vec::new();
static mut plotted_functions: Vec<Data> = Vec::new();



impl CliInterface {
    const UI_LEFT_MARGIN: u16 = 8;
    const UI_RIGHT_MARGIN: u16 = 6;

    // Header Space should be an even number
    const UI_HEADER_SPACE: u16 = 6;
    const UI_FOOTER_SPACE: u16 = 6;

    const MINIMUM_WIDTH: u16 = 85;
    const MINIMUM_HEIGHT: u16 = 60;

    const PLOT_GRAPH_CHARACTER: char = '*';



    /// Main loop of the cl_plotter, reads input and gives it to "process_input"
    pub fn cli_interface_loop() {
        let mut p = parser::Parser::new();

        Self::draw_screen(&"start".to_string());

        loop {
            let mut input = String::new();

            let _ = stdout().flush();

            println!("Type commands below!\n");
            stdin().read_line(&mut input).expect("Reading input failed!");

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

    /// Processes given input by user
    fn process_input(input: String, mut parser: Parser) -> Parser {
        let input_arguments: Vec<&str> = input.split(" ").collect();

        match &input_arguments.get(0).unwrap()[..] {
            "input" => {
                let given_function : String = input_arguments[1..].join(" ");
                if parser::Parser::count_occurrences('(', given_function.as_str()) != parser::Parser::count_occurrences(')', given_function.as_str())
                    || !Regex::new(r"([a-zA-Z]|(\d+\.?\d*)|[+\-/*]|\s)+").unwrap().is_match(given_function.as_str()) {
                    println!("Invalid input, try again!");
                } else {
                    unsafe { saved_functions.push((given_function.clone(), true)) }
                    println!("Insertion successful!\n\n");
                }
                    // parser.parse_expression(given_function);
                // parser.display_expression();
            }
            "ls" => unsafe {
                println!("All saved Functions:");
                for x in 0..saved_functions.len() {
                    let current_saved_func = saved_functions.get(x).unwrap();

                    let mut screen = String::new();
                    if current_saved_func.1 {
                        screen += "active"
                    } else {
                        screen += "not active"
                    }
                    println!(" {}: {} | {}", x, current_saved_func.0, screen);
                }
                print!("\n");
            }
            "rm" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                if input_arguments[1].parse::<u16>().unwrap() < 0 || input_arguments[1].parse::<u16>().unwrap() >= saved_functions.len() as u16 {
                    println!("Deletion index is wrong!");
                    return parser;
                }
                let remove_index = input_arguments[1].parse::<u16>().unwrap() as usize;
                saved_functions.remove(remove_index);

                println!("Deleted function at index: {}", remove_index);

                print!("\n");
            }
            "enable" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                if input_arguments[1].parse::<u16>().unwrap() < 0 || input_arguments[1].parse::<u16>().unwrap() >= saved_functions.len() as u16 {
                    println!("There is no function with given index!\n");
                    return parser;
                }
                let activate_index = input_arguments[1].parse::<u16>().unwrap() as usize;

                let mut to_be_activated_function = saved_functions[activate_index as usize].clone();
                to_be_activated_function.1 = true;
                saved_functions[activate_index as usize] = to_be_activated_function;

                println!("Enabled function at index: {}", activate_index);

                print!("\n");
            }
            "disable" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                if input_arguments[1].parse::<u16>().unwrap() < 0 || input_arguments[1].parse::<u16>().unwrap() >= saved_functions.len() as u16 {
                    println!("There is no function with given index!\n");
                    return parser;
                }
                let activate_index = input_arguments[1].parse::<u16>().unwrap() as usize;

                let mut to_be_disabled_function = saved_functions[activate_index as usize].clone();
                to_be_disabled_function.1 = false;
                saved_functions[activate_index as usize] = to_be_disabled_function;

                println!("Disabled function  {}", activate_index);

                print!("\n");
            }
            "plot" => unsafe {
                current_state = "plot";
                Self::draw_screen("plot")
            }
            "redraw" => unsafe {
                Self::draw_screen(current_state)
            }
            "xmin" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                let new_xmin = input_arguments[1].parse::<i64>().unwrap();

                if new_xmin >= X_MAX {
                    println!("X-Min cant be greater than X-Max, try something smaller.");
                    return parser;
                }
                X_MIN = new_xmin;
                if current_state == "plot" {
                    Self::draw_screen(current_state);
                } else {
                    println!("X-Min changed to {}.\n", new_xmin);
                }
                Self::print_current_graph_window()
            }
            "xmax" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                let new_xmax = input_arguments[1].parse::<i64>().unwrap();

                if new_xmax <= X_MIN {
                    println!("X-Max cant be smaller than X-Min, try something bigger.");
                    return parser;
                }
                X_MAX = new_xmax;
                if current_state == "plot" {
                    Self::draw_screen(current_state);
                } else {
                    println!("X-Max changed to {}.\n", new_xmax);
                }
                Self::print_current_graph_window()
            }
            "ymin" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                let new_ymin = input_arguments[1].parse::<i64>().unwrap();

                if new_ymin >= Y_MAX {
                    println!("Y-Min cant be greater than Y-Max, try something smaller.");
                    return parser;
                }
                Y_MIN = new_ymin;
                if current_state == "plot" {
                    Self::draw_screen(current_state);
                } else {
                    println!("Y-Min changed to {}.\n", new_ymin);
                }
                Self::print_current_graph_window()
            }
            "ymax" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                let new_ymax = input_arguments[1].parse::<i64>().unwrap();

                if new_ymax <= Y_MIN {
                    println!("Y-Max cant be smaller than Y-Min, try something bigger.");
                    return parser;
                }
                Y_MAX = new_ymax;
                if current_state == "plot" {
                    Self::draw_screen(current_state);
                } else {
                    println!("Y-Max changed to {}.\n", new_ymax);
                }
                Self::print_current_graph_window()
            }
            "fx" => unsafe {
                if input_arguments.len() < 3 {
                    println!("Too few arguments!");
                    return parser;
                } else if input_arguments.len() > 3 {
                    println!("Too much arguments!");
                    return parser;
                }

                let function_index = input_arguments[1].parse::<u16>().unwrap();
                if function_index < 0 || function_index >= saved_functions.clone().len() as u16 {
                    println!("Given Function does not exist!");
                }

                let x_value_to_calc_f_x = input_arguments[2].parse::<f32>().unwrap();

                let current_function = saved_functions.clone()[function_index as usize].clone().0;

                let mut p = Parser::new();

                p.parse_expression(current_function);
                let tmp = p.clone().stack.len();
                println!("Size of parser stack: {}", tmp);


                let data = data::Data::new(p.clone().stack.first().cloned().unwrap(),
                                           X_MIN as f32,
                                           X_MAX as f32,
                                           10);

                println!("Function value f({}) = {}\n", x_value_to_calc_f_x, data.evaluate_single(x_value_to_calc_f_x));
            }
            "min" => unsafe {
                if input_arguments.len() < 4 {
                    println!("Too few arguments!");
                    return parser;
                } else if input_arguments.len() > 4 {
                    println!("Too much arguments!");
                    return parser;
                }

                let function_index = input_arguments[1].parse::<u16>().unwrap();
                if function_index < 0 || function_index >= saved_functions.clone().len() as u16 {
                    println!("Given Function does not exist!");
                }

                let min_x_border = input_arguments[2].parse::<f32>().unwrap();
                let max_x_border = input_arguments[3].parse::<f32>().unwrap();

                let current_function = saved_functions.clone()[function_index as usize].clone().0;

                let mut p = Parser::new();

                p.parse_expression(current_function);


                let mut data = data::Data::new(p.clone().stack.first().cloned().unwrap(),
                                               min_x_border,
                                               max_x_border,
                                               10000);
                data.evaluate();

                println!("Minimum in Interval [{},{}] is {}\n", min_x_border, max_x_border, data.min(min_x_border, max_x_border));
            }
            "max" => unsafe {
                if input_arguments.len() < 4 {
                    println!("Too few arguments!");
                    return parser;
                } else if input_arguments.len() > 4 {
                    println!("Too much arguments!");
                    return parser;
                }

                let function_index = input_arguments[1].parse::<u16>().unwrap();
                if function_index < 0 || function_index >= saved_functions.clone().len() as u16 {
                    println!("Given Function does not exist!");
                }

                let min_x_border = input_arguments[2].parse::<f32>().unwrap();
                let max_x_border = input_arguments[3].parse::<f32>().unwrap();

                let current_function = saved_functions.clone()[function_index as usize].clone().0;

                let mut p = Parser::new();

                p.parse_expression(current_function);


                let mut data = data::Data::new(p.clone().stack.first().cloned().unwrap(),
                                               min_x_border,
                                               max_x_border,
                                               10000);
                data.evaluate();

                println!("Maximum in Interval [{},{}] is {}\n", min_x_border, max_x_border, data.max(min_x_border, max_x_border));
            }
            "integral" => unsafe {
                if input_arguments.len() < 4 {
                    println!("Too few arguments!");
                    return parser;
                } else if input_arguments.len() > 4 {
                    println!("Too much arguments!");
                    return parser;
                }

                let function_index = input_arguments[1].parse::<u16>().unwrap();
                if function_index < 0 || function_index >= saved_functions.clone().len() as u16 {
                    println!("Given Function does not exist!");
                }

                let min_x_border = input_arguments[2].parse::<f32>().unwrap();
                let max_x_border = input_arguments[3].parse::<f32>().unwrap();

                let current_function = saved_functions.clone()[function_index as usize].clone().0;

                let mut p = Parser::new();

                p.parse_expression(current_function);


                let mut data = data::Data::new(p.clone().stack.first().cloned().unwrap(),
                                               min_x_border,
                                               max_x_border,
                                               10000);
                data.evaluate();

                println!("Intergral in [{},{}] is about {}\n", min_x_border, max_x_border, data.integrate(min_x_border, max_x_border));
            }
            "diff" => unsafe {
                if input_arguments.len() < 2 {
                    println!("Too few arguments!");
                    return parser;
                }
                if input_arguments[1].parse::<u16>().unwrap() < 0 || input_arguments[1].parse::<u16>().unwrap() >= saved_functions.len() as u16 {
                    println!("Function cannot be found by index, check the indices by typing \"ls\"!");
                    return parser;
                }

                let size = terminal_size().unwrap().0;
                let width = size.0;


                let graph_width = width as u16 - (Self::UI_LEFT_MARGIN + Self::UI_RIGHT_MARGIN + 1);

                let func_idx = input_arguments[1].parse::<u16>().unwrap() as usize;

                let current_function = saved_functions.clone()[func_idx as usize].clone().0;

                let mut p = Parser::new();

                p.parse_expression(current_function);


                let mut data = data::Data::new(p.clone().stack.first().cloned().unwrap(),
                                               X_MIN as f32,
                                               X_MAX as f32,
                                               graph_width as usize);

                data.evaluate();
                data.differentiate();

                plotted_functions.push(data);

                Self::draw_screen("plot");
            }
            "help" => unsafe {
                current_state = "start";
                Self::draw_screen(current_state)
            }
            "window" => {
                Self::print_current_graph_window();
            }
            "exit" => {
                println!("Bye, have a beautiful time!");
                std::process::exit(0);
            }
            _ => println!("Unknown command, type \"help\" for a List of available commands!")
        }

        parser
    }


    /// Prints the current display area borders of the plot graph
    fn print_current_graph_window() {
        unsafe {
            println!("X-Min: {}  X-Max: {}  \nY-Min: {}  Y-Max: {}\n",
                     X_MIN, X_MAX, Y_MIN, Y_MAX);
        }
    }


    /// Method to draw either menu or plot screen
    fn draw_screen(s: &str) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            if w < Self::MINIMUM_WIDTH || h < Self::MINIMUM_HEIGHT{
                println!("Please make the terminal window bigger, it is too small at the moment");
                //TODO check height too
                std::process::exit(0);
            }

            match &s[..] {
                "start" => Self::draw_start_screen(w, h),
                "plot" => Self::draw_plot(w, h),

                _ => println!("Unknown draw screen command!")
            }
        }
    }


    /// Print the Starting Menu Screen
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
                           String::from("Enable or disable function from being plotted:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"enable <function-index> | disable <function-index>\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can remove a function from the list:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"rm <function-index>\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Get Function value of specific X-Value:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"fx <function-index> <X-Value>\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Get minimum or maximum in given interval:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from(format!("{}{}{}",
                                                "\"min <function-index> <left-border> <right-border>\"\n",
                                                left_padding.clone(),
                                                "\"max <function-index> <left-border> <right-border>\" \n\n")));
        printed_lines += 4;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Integrate function in given interval:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"integral <function-index> <left-border> <right-border>\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Differentiate a function and plot it:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"diff <function-index>\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Show the current graph window by typing:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"window\"\n\n"));
        printed_lines += 3;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Change the current graph window by typing:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"xmin <value>, xmax <value>, ymin <value>, ymax <value>\"\n\n"));
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
                           String::from("You can always come back to this page by typing:\n"));

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"help\"\n\n"));
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


    /// Super Method for drawing plot window with all activated functions
    fn draw_plot(width: u16, height: u16) {
        let mut screen = String::new();

        let graph_width = width - (Self::UI_LEFT_MARGIN + Self::UI_RIGHT_MARGIN + 1);
        let graph_height = height - (Self::UI_HEADER_SPACE + Self::UI_FOOTER_SPACE + 3);

        let mut p = Parser::new();

        unsafe {
            // let mut parser_stack;

            for (function, boolean) in saved_functions.clone() {
                if boolean {
                    p.parse_expression(function.clone());

                    let mut current_data = data::Data::new(p.clone().stack.first().cloned().unwrap(),
                                                           X_MIN as f32,
                                                           X_MAX as f32,
                                                           graph_width as usize);
                    current_data.evaluate();
                    plotted_functions.push(current_data);
                    p.clear();
                }
            }
        }

        let mut data_matrix = vec![vec![" "; graph_width as usize]; graph_height as usize];

        unsafe {
            for plot in plotted_functions.iter() {
                data_matrix = Self::plot_values_into_matrix(data_matrix,
                                                            plot.data.clone(),
                                                            height);
            }
        }

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

        unsafe { plotted_functions.clear() }
    }


    /// Parses given data vectors into data matrix in which the graphs are defined
    fn plot_values_into_matrix(mut data_matrix: Vec<Vec<&str>>, function_vec: Vec<f32>, height: u16) -> Vec<Vec<&str>> {
        let graph_height = height - (Self::UI_HEADER_SPACE + Self::UI_FOOTER_SPACE + 3);

        let graph_height_per_pixel: f64;
        unsafe { graph_height_per_pixel = (Y_MIN + Y_MAX) as f64 / graph_height as f64; };

        for func_value_index in 0..function_vec.len() {
            let func_value = function_vec.get(func_value_index).unwrap();

            for i in 0..graph_height {
                unsafe {
                    let current_y = Y_MIN as f64 + (graph_height_per_pixel * i as f64);
                    let next_y = Y_MIN as f64 + (graph_height_per_pixel * (i + 1) as f64);

                    if current_y < *func_value as f64 && *func_value as f64 <= next_y {
                        data_matrix[i as usize][func_value_index] = "*";
                    }
                }
            }
        }
        data_matrix
    }

    /// Creates Strings row for given row which are printed together as the full graph
    fn draw_graph_line(width: u16, height: u16, current_row: u16, data_matrix: Vec<Vec<&str>>) -> String {
        let mut screen = String::new();

        let graph_height = height - (Self::UI_HEADER_SPACE + Self::UI_FOOTER_SPACE + 3);


        if graph_height as i16 - (current_row as i16 - Self::UI_HEADER_SPACE as i16) < 0 {
            return "".to_string();
        }

        let current_height_relative_to_graph = graph_height - (current_row - Self::UI_HEADER_SPACE);


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
        String::from("\n")
    }


    fn draw_x_axis(width: i32) -> String {
        format!("{}{}{}",
                iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>(),
                "|",
                iter::repeat("_").take(((width - 1) - Self::UI_LEFT_MARGIN as i32 - Self::UI_RIGHT_MARGIN as i32) as usize).collect::<String>() + "\n")
    }

    // Sadly didnt have the time to implement correctly
    fn draw_x_axis_legend(width: i32) -> String {
        format!("{}{}",
                iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>(),
                iter::repeat("    |").take(((width - Self::UI_LEFT_MARGIN as i32 - Self::UI_RIGHT_MARGIN as i32) / 5) as usize).collect::<String>() + "\n")
    }

    // Sadly didnt have the time to implement correctly
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