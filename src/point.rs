#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub m_x: usize,
    pub m_y: usize,
}

impl Point {
    pub fn new(f_x: usize, f_y: usize) -> Self {
        Point {
            m_x: f_x,
            m_y: f_y 
        }
    }

    pub fn set_origin(&mut self, f_origin: &Point) {
        self.m_x = self.m_x + f_origin.m_x;
        self.m_y = self.m_y + f_origin.m_y;
    }
}

impl From<Point> for (i32, i32) {
    fn from(f_point: Point) -> (i32, i32) {
    (f_point.m_x as i32, f_point.m_y as i32)
    }
}