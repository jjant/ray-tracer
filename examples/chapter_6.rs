use examples::output_file_path;
use ray_tracer::{
    canvas::Canvas,
    color::Color,
    intersection::Intersection,
    light::Light,
    material::{self, Material},
    math::tuple::Tuple,
    ray::Ray,
    shape::Object,
};
use std::{fs::File, io::Write};

pub fn scene(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let sphere = Object::sphere();
    let mut material = Material::new();
    material.color = Color::new(1., 0.2, 1.);

    let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::white());

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

            if let Some(&hit) = Intersection::hit(&intersections) {
                let hit_point = r.position(hit.t);
                let hit_normal_vector = hit.object.normal_at(hit, hit_point);
                let eye = -r.direction;

                let pixel_color = material::lighting(
                    material,
                    hit.object,
                    light,
                    hit_point,
                    eye,
                    hit_normal_vector,
                    false,
                );

                canvas.write_pixel(x as i32, y as i32, pixel_color);
            }
        }
    }

    canvas
}

const ASPECT: f64 = 1. / 1.;

const WIDTH: usize = 300;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

pub fn main() {
    let file_name = output_file_path("chapter_6");
    println!("Writing scene to: {}", file_name);

    let canvas = scene(WIDTH, HEIGHT);
    let ppm = canvas.to_ppm();

    let mut f = File::create(&file_name).expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
