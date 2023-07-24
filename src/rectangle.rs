use crate::point::Point;

#[derive(Clone, Copy)]
pub struct Rectangle {
    m_height: usize,
    m_width: usize,
    pub m_origin: Point,
}

impl Rectangle {
    pub fn new(f_height: usize, f_width: usize, f_origin: &Point) -> Self {
        Rectangle {
            m_height: f_height,
            m_width: f_width,
            m_origin: f_origin.clone(),
        }
    }

    pub fn get_bottom_right_corner(&self) -> Point {
        let x_coord = &self.m_origin.m_x + &self.m_height;
        let y_coord = &self.m_origin.m_y + &self.m_width;

        return Point::new(x_coord, y_coord);
    }
}