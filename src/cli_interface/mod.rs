use std::io::{stdin, stdout, Write};
use std::ops::Rem;

use terminal_size::{Height, terminal_size, Width};

use super::data;
use super::parser;

pub struct CliInterface {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
    pub calculation_density: i64,

}

impl CliInterface {
    const UI_LEFT_MARGIN: u16 = 5;
    // Header Space should be an even number
    const UI_HEADER_SPACE: u16 = 6;
    const UI_FOOTER_SPACE: u16 = 3;
    const MINIMUM_WIDTH: u16 = 85;

    pub fn cli_interface_loop() {
        let mut p = parser::Parser::new();

        loop {
            let mut input = String::new();
            Self::draw_screen(&"start".to_string());
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

            // Fill data
            data::Data::new(p.clone().stack.first().unwrap()).evaluate();

            p.clear();
        }
    }


    pub fn render_functions() {
        // functions: Vec<fn(i64) -> i64>

        // let first_function = functions.first().expect("Failed");
        Self::draw_ui()
    }


    fn draw_screen(s: &str) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            if w < Self::MINIMUM_WIDTH {
                panic!("Terminal window too small please make it wider!")
            }

            match &s[..] {
                "start" => Self::draw_start_screen(w, h),
                _ => println!("Unknown draw screen command!")
            }
        }
    }


    fn draw_start_screen(width: u16, height: u16) {
        let mut screen = String::new();
        let mut printed_lines = 0;

        // Draw Header Line
        screen += &std::iter::repeat("_").take(width as usize).collect::<String>();
        screen += &String::from("\n");

        // Draw Header Space above Headline
        for _ in 1..(CliInterface::UI_HEADER_SPACE / 2) {
            screen += &String::from("\n");
        }

        // Draw Headline
        screen += &Self::generate_centered_text_string(width, &String::from("Welcome to the cl_plotter!"));

        // Draw Header Space below Headline
        for _ in 0..(CliInterface::UI_HEADER_SPACE / 2){
            screen += &String::from("\n");
        }

        let main_body_height = CliInterface::UI_HEADER_SPACE + CliInterface::UI_FOOTER_SPACE + 2;
        // TODO do this more often
        let left_padding = std::iter::repeat(" ").take(10).collect::<String>();
        // Draw Main Body
        for _ in 0..(height - main_body_height) {
            screen += &format!("{}{}", left_padding.clone(), String::from("main\n"));
            printed_lines += 1;
        }


        // Draw Footer
        for _ in 0..CliInterface::UI_FOOTER_SPACE {
            screen += &String::from("\n");
        }
        // for _ in 0..width {
        //     screen += &String::from("_");
        // }
        // screen += &String::from("\n");
        // printed_lines += 1;


        print!("{}", screen);
        // println!("printed Lines {}", printed_lines);
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


    fn draw_ui() {
        Self::draw_frame()
    }

    fn draw_frame() {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            for x in 0..h - 1 {
                if x < Self::UI_HEADER_SPACE {
                    Self::draw_header()
                } else if x == h - (Self::UI_FOOTER_SPACE + 3) {
                    Self::draw_x_axis(w as i32)
                } else if x == h - (Self::UI_FOOTER_SPACE + 2) {
                    Self::draw_x_axis_legend(w as i32)
                } else if x == h - (Self::UI_FOOTER_SPACE + 1) {
                    Self::draw_x_axis_legend_numbers(w as i32)
                } else if x >= h - Self::UI_FOOTER_SPACE {
                    Self::draw_footer()
                } else {
                    println!("  |");
                }
            }
        } else {
            println!("Unable to get terminal size");
        }
    }

    fn draw_header() {
        println!("Header")
    }

    fn draw_footer() {
        println!("Footer")
    }

    fn draw_x_axis(width: i32) {
        print!("  ");

        for x in 2..width / 2 {
            print!("__");
        }
        println!()
    }

    fn draw_x_axis_legend(width: i32) {
        print!("  ");

        for x in 2..width / 4 {
            print!("   |");
        }
        println!()
    }

    fn draw_x_axis_legend_numbers(width: i32) {
        print!("  ");

        for x in 2..width / 4 {
            if x < 10 {
                print!("   {}", x);
            } else {
                print!("  {}", x);
            }
        }
        println!()
    }

    fn plot_graph() {}
}