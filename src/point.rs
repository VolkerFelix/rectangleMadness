#[derive(Debug, Clone, Copy)]
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
}
