mod rectangle;
mod canvas;
mod point;
mod auxiliary;
fn main() {
    println!("Hello, world!");

    let mut canvas = canvas::Canvas::new();

    for _i in 0..2 {
        let random_height = auxiliary::generate_random_value(usize::MAX);
        let random_width = auxiliary::generate_random_value(usize::MAX);

        let mut rect = rectangle::Rectangle::new(random_height, random_width, &canvas.m_origin);

        canvas.add_rectangle(&mut rect);
    }

    println!("Origin of last rect is: {:?}", canvas.m_rectangles.last().unwrap().m_origin);
}
