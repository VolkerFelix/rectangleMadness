use crate::{
    rectangle::Rectangle,
    MIN_HEIGHT_RECTANGLE_PX, MIN_WIDTH_RECTANGLE_PX, MAX_HEIGHT_RECTANGLE_PX, MAX_WIDTH_RECTANGLE_PX,
    horizontalizer::{Horizontalizer, self},
    canvas::Canvas,
};

#[test]
fn areas_of_vertical_and_horizontal_rectangles_are_the_same() {
    let nr_of_rectangles = 5;
    let mut canvas = Canvas::new();

    for _ in 0..nr_of_rectangles {
        let mut rect = Rectangle::generate_random_rectangle(
            MIN_HEIGHT_RECTANGLE_PX, MAX_HEIGHT_RECTANGLE_PX,
            MIN_WIDTH_RECTANGLE_PX, MAX_WIDTH_RECTANGLE_PX);
        canvas.add_rectangle_and_update_origin(&mut rect);
    }

    let mut horizontalizer = Horizontalizer::new(&canvas.m_rectangles);
    horizontalizer.run();

    let mut vertical_area = 0;
    for v_rect in &canvas.m_rectangles{
        vertical_area = vertical_area + v_rect.get_area();
    }

    let mut horizontal_area = 0;
    for h_rect in horizontalizer.m_horizontal_rectangles {
        horizontal_area = horizontal_area + h_rect.get_area();
    }

    assert_eq!(vertical_area, horizontal_area);
}