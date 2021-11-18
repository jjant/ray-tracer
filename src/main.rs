mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix2;
mod matrix3;
mod matrix4;
mod misc;
mod ray;
mod shape;
mod sphere;
mod transformations;
mod tuple;
mod world;

use canvas::Canvas;
use color::Color;
use light::Light;
use matrix4::Matrix4;
use sphere::Sphere;
use std::f64::consts::PI;
use tuple::Tuple;

use crate::{camera::Camera, shape::Object, world::World};

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

fn draw_world() -> Canvas {
    let mut floor = Object::sphere();
    floor.set_transform(Matrix4::scaling(10., 0.01, 10.));
    floor.material_mut().color = Color::new(1., 0.9, 0.9);
    floor.material_mut().specular = 0.;

    let mut left_wall = Object::sphere();
    left_wall.set_transform(
        Matrix4::translation(0., 0., 5.)
            * Matrix4::rotation_y(-PI / 4.)
            * Matrix4::rotation_x(PI / 2.)
            * Matrix4::scaling(10., 0.01, 10.),
    );
    *left_wall.material_mut() = floor.material();

    let mut right_wall = Object::sphere();
    *right_wall.transform_mut() = Matrix4::translation(0., 0., 5.)
        * Matrix4::rotation_y(PI / 4.)
        * Matrix4::rotation_x(PI / 2.)
        * Matrix4::scaling(10., 0.01, 10.);
    *right_wall.material_mut() = floor.material();

    let mut middle = Object::sphere();
    *middle.transform_mut() = Matrix4::translation(-0.5, 1., 0.5);
    middle.material_mut().color = Color::new(0.1, 1., 0.5);
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;

    let mut right = Object::sphere();
    *right.transform_mut() = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    right.material_mut().color = Color::new(0.5, 1., 0.1);
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;

    let mut left = Object::sphere();
    *left.transform_mut() =
        Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    left.material_mut().color = Color::new(1., 0.8, 0.1);
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;

    let mut world = World::new();
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];
    world.light = Some(Light::point_light(
        Tuple::point(-10., 10., -10.),
        Color::white(),
    ));

    let mut camera = Camera::new(WIDTH as i32, HEIGHT as i32, PI / 3.);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 1.5, -5.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    camera.render(&world)
}

fn main() {
    println!("{}", draw_world().to_ppm());
}
