use terminal_size::{Height, terminal_size, Width};

pub struct CliInterface {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
    pub calculation_density: i64,
}

impl CliInterface {

    const UI_LEFT_MARGIN: u16 = 5;
    const UI_HEADER_SPACE: u16 = 10;
    const UI_FOOTER_SPACE: u16 = 5;


    pub fn render_functions() {
        // functions: Vec<fn(i64) -> i64>

        // let first_function = functions.first().expect("Failed");
        Self::draw_ui()
    }

    fn draw_ui() {
        Self::draw_frame()
    }

    fn draw_frame() {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            for x in 0..h - 1 {
                if (x < Self::UI_HEADER_SPACE) {
                    Self::draw_header()
                } else if (x == h - (Self::UI_FOOTER_SPACE + 3)) {
                    Self::draw_x_axis(w as i32)
                } else if (x == h - (Self::UI_FOOTER_SPACE + 2)) {
                    Self::draw_x_axis_legend(w as i32)
                } else if (x == h - (Self::UI_FOOTER_SPACE + 1)) {
                    Self::draw_x_axis_legend_numbers(w as i32)
                } else if (x >= h - Self::UI_FOOTER_SPACE) {
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
        println!("")
    }

    fn draw_x_axis_legend(width: i32) {
        print!("  ");

        for x in 2..width / 4 {
            print!("   |");
        }
        println!("")
    }

    fn draw_x_axis_legend_numbers(width: i32) {
        print!("  ");

        for x in 2..width / 4 {
            if (x < 10) {
                print!("   {}", x);
            } else {
                print!("  {}", x);
            }
        }
        println!("")
    }

    fn plot_graph() {}
}