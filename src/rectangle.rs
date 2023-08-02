use crate::{
    point::Point,
    auxiliary,
    output::RectangleOutputFormat,
};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Rectangle {
    pub m_top_left: Point,
    pub m_bottom_right: Point,
}

impl Rectangle {
    pub fn new(f_top_left: &Point, f_bottom_right: &Point) -> Self {
        Rectangle {
            m_top_left: f_top_left.clone(),
            m_bottom_right: f_bottom_right.clone(),
        }
    }

    pub fn get_top_right_corner(&self) -> Point {
        Point::new(self.m_bottom_right.m_x, self.m_top_left.m_y)

    }

    pub fn generate_random_rectangle(f_min_height_px: usize, f_max_height_px: usize, f_min_width_px: usize, f_max_width_px: usize) -> Rectangle {
        let bottom_right_x = auxiliary::generate_random_value(f_min_height_px, f_max_height_px);
        let bottom_right_y = auxiliary::generate_random_value(f_min_width_px, f_max_width_px);

        let bottom_right = Point::new(bottom_right_x, bottom_right_y);

        Rectangle::new(&Point::default(), &bottom_right)
    }

    pub fn set_origin(&mut self, f_origin: &Point) {
        self.m_top_left.set_origin(f_origin);
        self.m_bottom_right.set_origin(f_origin);
    }

    pub fn get_height(&self) -> usize {
        self.m_bottom_right.m_y - self.m_top_left.m_y
    }

    pub fn get_width(&self) -> usize {
        self.m_bottom_right.m_x - self.m_top_left.m_x
    }

    pub fn get_area(&self) -> usize {
        self.get_height() * self.get_width()
    }
}

impl From<&Rectangle> for RectangleOutputFormat {
    fn from(f_rectangle: &Rectangle) -> RectangleOutputFormat  {
        RectangleOutputFormat {
            x: f_rectangle.m_top_left.m_x,
            y: f_rectangle.m_top_left.m_y,
            width: f_rectangle.get_width(),
            height: f_rectangle.get_height()
        }
    }
}