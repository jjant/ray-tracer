mod camera;
mod canvas;
mod color;
mod examples;
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
mod tuple;
mod world;

use canvas::Canvas;
use color::Color;
use light::Light;
use matrix4::Matrix4;
use std::f64::consts::PI;
use tuple::Tuple;

use crate::{
    camera::Camera, material::Material, misc::degrees, pattern::Pattern, shape::Object,
    world::World,
};

const WIDTH: usize = 1000;
const HEIGHT: usize = 625;

fn draw_world() -> Canvas {
    let mut floor = Object::plane();
    *floor.material_mut() =
        Material::with_pattern(Pattern::checkered(Color::black(), Color::white()));
    floor.material_mut().color = Color::new(1., 0.9, 0.9);
    floor.material_mut().specular = 0.;

    let mut middle = Object::sphere();
    *middle.transform_mut() = Matrix4::translation(-0.5, 1., 0.5);
    let mut pattern = Pattern::ring(Color::rgb255(0, 240, 10), Color::rgb255(10, 200, 25));
    *pattern.transform_mut() = Matrix4::rotation_z(degrees(35.))
        * Matrix4::rotation_x(degrees(-60.))
        * Matrix4::scaling(0.45, 0.45, 0.45);
    middle.material_mut().color = Color::new(0.1, 1., 0.5);
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;
    middle.material_mut().reflective = Some(1.0);

    let mut right = Object::sphere();
    *right.transform_mut() = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    right.material_mut().color = Color::new(0.5, 1., 0.1);
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;
    right.material_mut().reflective = Some(1.0);

    let mut left = Object::sphere();
    *left.transform_mut() =
        Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    left.material_mut().color = Color::new(1., 0.8, 0.1);
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;
    left.material_mut().reflective = Some(1.0);

    let mut world = World::new();
    world.objects = vec![
        floor,
        middle,
        right,
        left,
        examples::back_wall(),
        examples::right_wall(),
    ];
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
