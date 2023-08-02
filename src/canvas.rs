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
            m_origin: Point::default(),
            m_rectangles: vec![],
        }
    }

    pub fn add_rectangle_and_update_origin(&mut self, f_rectangle: &mut Rectangle) {
        f_rectangle.set_origin(&self.m_origin);
        self.m_rectangles.push(f_rectangle.clone());
        self.set_next_origin_for_new_rectangle();
    }

    fn set_next_origin_for_new_rectangle(&mut self) {
        self.m_origin = self.m_rectangles.last().unwrap().get_top_right_corner();
    }
}