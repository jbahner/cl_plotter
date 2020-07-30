use std::fmt::format;
use std::io::{stdin, stdout, Write};
use std::iter;
use std::ops::Rem;
use std::thread::current;

use terminal_size::{Height, terminal_size, Width};

use crate::parser::Parser;

use super::data;
use super::parser;

pub struct CliInterface {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
    pub calculation_density: i64,

}

thread_local! {static mut current_state: &str = "start";}

impl CliInterface {
    const UI_LEFT_MARGIN: u16 = 8;
    // Header Space should be an even number
    const UI_HEADER_SPACE: u16 = 6;
    const UI_FOOTER_SPACE: u16 = 6;
    const MINIMUM_WIDTH: u16 = 85;


    pub fn cli_interface_loop() {
        let mut p = parser::Parser::new();
        // TODO change this back to "start"

        Self::draw_screen(current_state);

        loop {
            let mut input = String::new();

            let _ = stdout().flush();

            println!("Type commands below!\n");
            stdin().read_line(&mut input).expect("Retard!");

            if let Some('\n') = input.chars().next_back() {
                input.pop();
            }
            if let Some('\r') = input.chars().next_back() {
                input.pop();
            }

            // TODO: Validate input format: only one variable allowed
            // TODO: here comes parsing of typed in command

            p = Self::process_input(input, p);


            // Fill data
            match p.clone().stack.first() {
                None => continue,
                Some(data) => data::Data::new(data).evaluate()
            }


            p.clear();
        }
    }

    fn process_input(input: String, mut parser: Parser) -> Parser {
        let input_arguments: Vec<&str> = input.split(" ").collect();

        match &input_arguments.get(0).unwrap()[..] {
            "input" => {
                let given_function = input_arguments[1..].join(" ");
                parser.parse_expression(given_function);
                parser.display_expression();
            }
            "plot" => {
                Self::draw_screen("plot")
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


        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("You can redraw the current Ui:\n"));
        printed_lines += 1;

        screen += &format!("{}{}",
                           left_padding.clone(),
                           String::from("TODO \"redraw\"\n\n"));
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
            for x in 0..height - 3 {
                if x < Self::UI_HEADER_SPACE {
                    screen += &Self::draw_header(x, width)
                } else if x == height - (Self::UI_FOOTER_SPACE + 3) {
                    screen += &Self::draw_x_axis(width as i32)
                } else if x == height - (Self::UI_FOOTER_SPACE + 2) {
                    screen += &Self::draw_x_axis_legend(width as i32)
                } else if x == height - (Self::UI_FOOTER_SPACE + 1) {
                    screen += &Self::draw_x_axis_legend_numbers(width as i32)
                } else if x >= height - Self::UI_FOOTER_SPACE {
                    screen += &Self::draw_footer()
                } else {
                    screen += "  |\n";
                }
            }
            print!("{}", screen);
        } else {
            println!("Unable to get terminal size");
        }
    }

    fn draw_header(current_height: u16, width: u16) -> String {
        if current_height == 2 {
            return format!("{}", Self::generate_centered_text_string(width, "Your Plot"));
        }
        return format!("{}", "\n");
    }

    fn draw_footer() -> String {
        return String::from("Footer\n");
    }

    fn draw_x_axis(width: i32) -> String {
        // TODO Padding here
        return format!("{}{}",
                       "  ",
                       iter::repeat("_").take((width - 2) as usize).collect::<String>() + "\n");

        // print!("  ");
        //
        // for x in 2..width / 2 {
        //     print!("__");
        // }
        // println!()
    }

    fn draw_x_axis_legend(width: i32) -> String {
        return format!("{}{}",
                       "  ",
                       iter::repeat("   |").take(((width - 2) / 4) as usize).collect::<String>() + "\n");
        // print!("  ");
        //
        // for x in 2..width / 4 {
        //     print!("   |");
        // }
        // println!()
    }

    fn draw_x_axis_legend_numbers(width: i32) -> String {
        let mut screen = String::new();

        screen += "  ";

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