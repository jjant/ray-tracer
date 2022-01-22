mod camera;
mod canvas;
mod color;
mod examples;

mod intersection;
mod light;
mod material;
mod misc;
mod pattern;
mod ray;
mod shape;
mod world;
use examples::{chapter_11, chapter_12, chapter_13, chapter_14, chapter_15};
use std::fs::File;
use std::io::Write;
mod math;

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 400;
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
