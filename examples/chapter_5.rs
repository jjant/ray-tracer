use examples::output_file_path;
use ray_tracer::{
    canvas::Canvas, color::Color, intersection::Intersection, math::tuple::Tuple, ray::Ray,
    shape::Object,
};
use std::{fs::File, io::Write};

pub fn scene(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let sphere = Object::sphere();
    let red = Color::red();

    let camera_origin = Tuple::point(0., 0., -5.);

    let wall_distance = 10.;
    let wall_size = 7.;
    let pixel_size = wall_size / canvas.width() as f64;

    for y in 0..canvas.height() {
        let world_y = (wall_size / 2.) - pixel_size * y as f64;
        for x in 0..canvas.width() {
            let world_x = -(wall_size / 2.) + pixel_size * x as f64;
            let position = Tuple::point(world_x, world_y, wall_distance);
            let r = Ray::new(camera_origin, (position - camera_origin).normalize());

            let intersections = sphere.intersect(r);

            if let Some(_) = Intersection::hit(&intersections) {
                canvas.write_pixel(x as i32, y as i32, red);
            }
        }
    }

    canvas
}

const ASPECT: f64 = 1. / 1.;

const WIDTH: usize = 300;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

pub fn main() {
    let file_name = output_file_path("chapter_5");
    println!("Writing scene to: {}", file_name);

    let canvas = scene(WIDTH, HEIGHT);
    let ppm = canvas.to_ppm();

    let mut f = File::create(&file_name).expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
