mod canvas;
mod color;
mod light;
mod material;
mod matrix2;
mod matrix3;
mod matrix4;
mod misc;
mod ray;
mod sphere;
mod tuple;

use canvas::Canvas;
use color::Color;
use light::Light;
use matrix4::Matrix4;
use ray::{Intersection, Ray};
use sphere::Sphere;
use std::f64::consts::PI;
use tuple::Tuple;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn draw_sphere(canvas: &mut Canvas) {
    let mut sphere = Sphere::new();

    sphere.material.color = Color::new(1., 0.2, 1.);

    let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::white());

    // shrink it along the y axis
    // sphere.set_transform(Matrix4::scaling(1., 0.5, 1.));

    // shrink it along the x axis
    // sphere.set_transform(Matrix4::scaling(0.5, 1., 1.));

    // shrink it, and rotate it!
    sphere.set_transform(Matrix4::rotation_z(PI / 4.) * Matrix4::scaling(0.5, 1., 1.));

    // shrink it, and skew it!
    // sphere.set_transform(Matrix4::shearing(1., 0., 0., 0., 0., 0.) * Matrix4::scaling(0.5, 1., 1.));

    let wall_z = 10.;
    let wall_size = 7.;
    let pixel_size = wall_size / WIDTH as f64;
    let half = wall_size / 2.;

    for y in 0..HEIGHT {
        let world_y = half - pixel_size * y as f64;
        for x in 0..WIDTH {
            let world_x = -half + pixel_size * x as f64;

            let wall_point = Tuple::point(world_x, world_y, wall_z);

            let ray_origin = Tuple::point(0., 0., -5.);
            let ray = Ray::new(ray_origin, (wall_point - ray_origin).normalize());

            let xs = ray.intersect(sphere);

            if let Some(hit) = Intersection::hit(&xs) {
                let point = ray.position(hit.t);
                let normal = sphere.normal_at(point);
                let eye = -ray.direction;

                let color = material::lighting(hit.object.material, light, point, eye, normal);
                canvas.write_pixel(x as i32, y as i32, color)
            } else {
                canvas.write_pixel(x as i32, y as i32, Color::black())
            }
        }
    }
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    draw_sphere(&mut canvas);
    println!("{}", canvas.to_ppm());
}
