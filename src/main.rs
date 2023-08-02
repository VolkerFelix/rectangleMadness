mod rectangle;
mod canvas;
mod point;
mod auxiliary;
mod horizontalizer;
mod output;
#[cfg(test)]
mod tests;

use std::error::Error;
use clap::Parser;

const MIN_NR_RECTANGLES:usize = 5;
const MAX_NR_RECTANLGES:usize = 15;
// Define some min and max values to have the rectangles in more or less a similar shape.
// Not needed though - just to look better.
const MAX_HEIGHT_RECTANGLE_PX:usize = 150;
const MIN_HEIGHT_RECTANGLE_PX:usize = 10;
const MAX_WIDTH_RECTANGLE_PX:usize = MAX_HEIGHT_RECTANGLE_PX;
const MIN_WIDTH_RECTANGLE_PX:usize = MIN_HEIGHT_RECTANGLE_PX;


#[derive(Parser)]
struct Cli {
    /// Number of rectangles to generate: [5 <= n <= 15]
    #[arg(short, long)]
    number_of_rectangles: usize,
    /// Plot the result
    #[arg(short, long)]
    plot_result: bool,
}
impl Cli {
    fn check_arguments(&self) {
        assert!(self.number_of_rectangles >= MIN_NR_RECTANGLES && self.number_of_rectangles <= MAX_NR_RECTANLGES,
        "Number of rectangles n must be wihtin 5 <= n <= 15.");
    }
}
fn main() -> Result<(), Box<dyn Error>>{
    let args = Cli::parse();
    args.check_arguments();

    let mut canvas = canvas::Canvas::new();

    for _ in 0..args.number_of_rectangles {
        let mut rect = rectangle::Rectangle::generate_random_rectangle(
            MIN_HEIGHT_RECTANGLE_PX, MAX_HEIGHT_RECTANGLE_PX,
            MIN_WIDTH_RECTANGLE_PX, MAX_WIDTH_RECTANGLE_PX);
        canvas.add_rectangle_and_update_origin(&mut rect);
    }

    let mut horizontalizer = horizontalizer::Horizontalizer::new(&canvas.m_rectangles);
    horizontalizer.run();

    // Put the rectangles in the desired format
    let mut source_rectangles: Vec<output::RectangleOutputFormat> = vec![];
    for s_rect in &canvas.m_rectangles {
        source_rectangles.push(s_rect.into());
    }
    let mut target_rectangles: Vec<output::RectangleOutputFormat> = vec![];
    for t_rect in &horizontalizer.m_horizontal_rectangles {
        target_rectangles.push(t_rect.into());
    }

    let output = output::Output::new(args.number_of_rectangles, source_rectangles, target_rectangles);
    output.save_result();

    if args.plot_result {
        auxiliary::plot_output(&canvas.m_rectangles, &horizontalizer.m_horizontal_rectangles)?;
    }

    Ok(())
}
