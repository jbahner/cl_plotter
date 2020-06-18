use terminal_size::{Height, terminal_size, Width};

pub struct CliInterface {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
    pub calculation_density: i64,
}

impl CliInterface {
    // use terminal_size::{Width, Height, terminal_size};

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
                if (x < 5) {
                    Self::draw_header()
                } else if (x == h - 5) {
                    Self::draw_x_axis(w as i32)
                } else if (x > h - 5) {
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

    fn plot_graph() {}
}