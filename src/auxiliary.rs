use rand::{self, Rng};
use std::error::Error;
use plotters::prelude::*;
use crate::rectangle::Rectangle;

pub fn generate_random_value(f_min_value: usize, f_max_value: usize) -> usize {
    rand::thread_rng().gen_range(f_min_value..f_max_value)
}

pub fn plot_output(f_vertical_rectangles: &Vec<Rectangle>, f_horizontal_rectangles: &Vec<Rectangle>) -> Result<(), Box<dyn Error>> {

    let mut total_width = 0;
    for v_rects in f_vertical_rectangles {
        total_width = total_width + v_rects.get_width();
    }

    let max_height = f_vertical_rectangles
        .iter()
        .max_by(|a, b| {
        a.m_bottom_right.m_y.cmp(&b.m_bottom_right.m_y)
        })
        .unwrap()
        .get_height();

    let mut backend = BitMapBackend::new(
        "rectangles.png",
        (total_width as u32, max_height as u32));

    for v_rect in f_vertical_rectangles {
        backend.draw_rect(
            v_rect.m_top_left.into(),
            v_rect.m_bottom_right.into(),
            &RED, true)?;
    }

    for h_rect in f_horizontal_rectangles {
        backend.draw_rect(
            h_rect.m_top_left.into(),
            h_rect.m_bottom_right.into(),
            &BLUE, false)?;
    }
    
    backend.present()?;

    Ok(())
}