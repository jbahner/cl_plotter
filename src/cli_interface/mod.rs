use std::borrow::{Borrow, BorrowMut};
use std::fmt::format;
use std::io::{stdin, stdout, Write};
use std::iter;
use std::ops::Rem;
use std::ptr::null;
use std::thread::current;

use terminal_size::{Height, terminal_size, Width};

use crate::parser::Parser;

use super::data;
use super::parser;

pub struct CliInterface {
    pub calculation_density: i64,

}

static mut current_state: &str = "start";
static mut X_MIN: i64 = 0;
static mut X_MAX: i64 = 10;
static mut Y_MIN: i64 = 0;
static mut Y_MAX: i64 = 20;

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

            let mut calculated_points: Vec<f64> = vec!();
            // Fill data
            // match p.clone().stack.first() {
            //     None => continue,
            //     // Some(stack) => data::Data::new(data).evaluate()
            // }


            p.clear();
        }
    }

    fn process_input(input: String, mut parser: Parser) -> Parser {
        let input_arguments: Vec<&str> = input.split(" ").collect();

        match &input_arguments.get(0).unwrap()[..] {
            "input" => {
                let given_function = input_arguments[1..].join(" ");
                // unsafe { func_vec.push(&given_function) }
                println!("Insertion successful!\n\n");
                // parser.parse_expression(given_function);
                // parser.display_expression();
            }
            "ls" => unsafe {
                // TODO implement ls
                println!("All saved Functions:");
                // for x in 0..func_vec.len() {
                //     println!("  {}{}", x, func_vec.get(x).unwrap());
                // }
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

        return parser;
    }


    fn draw_screen(s: &str) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            if w < Self::MINIMUM_WIDTH {
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
        printed_lines += 1;

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"input <your function>\"\n\n"));
        printed_lines += 2;

        //
        // screen += &format!("{}{}",
        //                    left_padding.clone(),
        //                    String::from("You can redraw the current Ui:\n"));
        // printed_lines += 1;
        //
        // screen += &format!("{}{}",
        //                    left_padding.clone(),
        //                    String::from("\"redraw\"\n\n"));
        // printed_lines += 2;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can redraw the current Ui:\n"));
        printed_lines += 1;

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"redraw\"\n\n"));
        printed_lines += 2;

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can plot the current function:\n"));
        printed_lines += 1;

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"plot\"\n\n"));
        printed_lines += 2;


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("Or exit by typing:\n"));
        printed_lines += 1;

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("\"exit\"\n\n"));
        printed_lines += 2;

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


    // fn draw_ui() {
    //     Self::draw_frame()
    // }

    fn draw_plot(width: u16, height: u16) {
        let size = terminal_size();
        let mut screen = String::new();

        if let Some((Width(width), Height(height))) = size {
            for current_height in 0..height - 3 {
                if current_height < Self::UI_HEADER_SPACE {
                    screen += &Self::draw_header(current_height, width)
                } else if current_height >= height - Self::UI_FOOTER_SPACE {
                    screen += &Self::draw_footer()
                } else {
                    screen += &Self::draw_graph_line(width, height, current_height);
                }
            }
            print!("{}", screen);
        } else {
            println!("Unable to get terminal size");
        }
    }


    fn draw_graph_line(width: u16, height: u16, current_row: u16) -> String {


        let mut screen = String::new();
        let graph_width = width - (Self::UI_LEFT_MARGIN + Self::UI_RIGHT_MARGIN + 1);
        let graph_height = height - (Self::UI_HEADER_SPACE + Self::UI_FOOTER_SPACE + 3);

        let function_vec = vec![15; graph_width as usize];



        let mut to_be_plotted_indices: Vec<u16> = vec!();




        if graph_height as i16 - (current_row as i16 - Self::UI_HEADER_SPACE as i16) >= 0 {
            let mut graph_height_per_pixel: f64;
            unsafe { graph_height_per_pixel =  (Y_MIN + Y_MAX) as f64 / graph_height as f64; };
            let current_height_relative_to_graph = graph_height - (current_row - Self::UI_HEADER_SPACE);
            let current_y_value = current_height_relative_to_graph as f64 * graph_height_per_pixel;
            // println!("Current Value: {}", current_y_value);
            println!("Current Position: {}", current_height_relative_to_graph);


            let next_y_value = (current_height_relative_to_graph as f64 - 1.) * graph_height_per_pixel;


            for i in 0..function_vec.len() {
                let current_func_value = *function_vec.get(i).unwrap() as f64;
                // println!("Current Func Val{}", current_func_value);

                if current_func_value > next_y_value && current_func_value <= current_y_value {
                    to_be_plotted_indices.push(i as u16)
                }
            }

            // for i in to_be_plotted_indices.clone() { println!("{}", i)}

            println!("Y-Val: {}, Number of values in row: {}\n", current_y_value, to_be_plotted_indices.len())


        }




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

            if to_be_plotted_indices.len() != 0 {
                let mut before_width_index = 0;

                for plot_val in to_be_plotted_indices {
                    screen += &(iter::repeat(" ").take(((plot_val + 1) - before_width_index - 1) as usize).collect::<String>());
                    screen += &Self::PLOT_GRAPH_CHARACTER.to_string();
                    before_width_index = plot_val + 1;
                }

            }

            screen += "\n";
        }
        return screen;
    }

    fn draw_header(current_height: u16, width: u16) -> String {
        if current_height == 2 {
            return format!("{}", Self::generate_centered_text_string(width, "Your Plot"));
        }
        return format!("{}", "\n");
    }

    fn draw_footer() -> String {
        return String::from("f\n");
    }

    fn draw_x_axis(width: i32) -> String {
        return format!("{}{}{}",
                       iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>(),
                       "|",
                       iter::repeat("_").take(((width - 1) - Self::UI_LEFT_MARGIN as i32 - Self::UI_RIGHT_MARGIN as i32) as usize).collect::<String>() + "\n");

        // print!("  ");
        //
        // for x in 2..width / 2 {
        //     print!("__");
        // }
        // println!()
    }

    fn draw_x_axis_legend(width: i32) -> String {
        return format!("{}{}",
                       iter::repeat(" ").take(Self::UI_LEFT_MARGIN as usize).collect::<String>(),
                       iter::repeat("    |").take(((width - Self::UI_LEFT_MARGIN as i32 - Self::UI_RIGHT_MARGIN as i32) / 5) as usize).collect::<String>() + "\n");
        // print!("  ");
        //
        // for x in 2..width / 4 {
        //     print!("   |");
        // }
        // println!()
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
        return screen;
    }
}