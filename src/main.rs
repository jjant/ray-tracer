mod tuple;
use tuple::Tuple;
mod canvas;
mod color;
mod misc;
use canvas::Canvas;
use color::Color;

fn main() {
    let mut canvas = Canvas::new(20, 20);

    canvas.write_pixel(10, 10, Color::white());

    println!("{}", canvas.to_ppm());
}
