mod canvas;
mod color;
mod misc;
mod tuple;
use canvas::Canvas;
use color::Color;

fn main() {
    let mut canvas = Canvas::new(20, 20);

    canvas.write_pixel(10, 10, Color::white());

    println!("{}", canvas.to_ppm());
}
