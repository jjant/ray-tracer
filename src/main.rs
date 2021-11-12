mod canvas;
mod color;
mod matrix2;
mod matrix3;
mod matrix4;
mod misc;
mod ray;
mod tuple;

use canvas::Canvas;
use color::Color;
use matrix4::Matrix4;
use std::f64::consts::PI;
use tuple::Tuple;

fn compute_clock() -> Vec<Tuple> {
    let mut hours = vec![];
    let twelve_position = Tuple::point(0., 0., 1.);

    for hour in 0..12 {
        let hour_position = Matrix4::rotation_y(PI / 6. * (hour as f64)) * twelve_position;

        hours.push(hour_position);
    }

    hours
}

fn main() {
    let width = 400;
    let height = 400;
    let mut canvas = Canvas::new(width, height);

    let r = 3. / 8. * width as f64;

    let clock = compute_clock();
    let transformation = Matrix4::translation(width as f64 / 2., 0., (height as f64) / 2.)
        * Matrix4::scaling(r, 0., r);

    clock
        .iter()
        .map(|point| transformation * *point)
        .for_each(|transformed_point| {
            canvas.write_pixel(
                transformed_point.x as i32,
                transformed_point.z as i32,
                Color::red(),
            )
        });

    println!("{}", canvas.to_ppm());
}
