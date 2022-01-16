mod camera;
mod canvas;
mod color;
mod examples;

mod cone;
mod cube;
mod cylinder;
mod intersection;
mod light;
mod material;
mod matrix2;
mod matrix3;
mod matrix4;
mod misc;
mod pattern;
mod plane;
mod ray;
mod shape;
mod sphere;
mod transformations;
mod triangle;
mod tuple;
mod world;
use examples::{chapter_11, chapter_12, chapter_13, chapter_14, chapter_15};
use std::fs::File;
use std::io::Write;

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 200;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

fn main() {
    let (_camera, _world) = chapter_11::scene(WIDTH, HEIGHT);
    let (_camera, _world) = chapter_12::scene(WIDTH, HEIGHT);
    let (_camera, _world) = chapter_13::scene(WIDTH, HEIGHT);
    let (_camera, _world) = chapter_14::scene(WIDTH, HEIGHT);
    let (camera, world) = chapter_15::scene(WIDTH, HEIGHT);
    let ppm = camera.render(&world).to_ppm();

    let mut f = File::create("./output.ppm").expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
