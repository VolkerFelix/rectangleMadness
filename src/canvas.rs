use crate::rectangle::Rectangle;
use crate::point::Point;

#[derive(Clone)]
pub struct Canvas {
    pub m_origin: Point,
    pub m_rectangles: Vec<Rectangle>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas {
            m_origin: Point::new(0, 0),
            m_rectangles: vec![],
        }
    }

    pub fn add_rectangle(&mut self, f_rectangle: &mut Rectangle) {
        if self.m_rectangles.is_empty() {
            // No previous rectangles there --> place at canvas origin
            f_rectangle.m_origin = self.m_origin;
        }
        else {
            f_rectangle.m_origin = self.m_rectangles.last().unwrap().get_bottom_right_corner();
        }
        self.m_rectangles.push(f_rectangle.clone());
    }
}