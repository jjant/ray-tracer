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
mod tuple;
mod world;

use color::Color;
#[allow(unused_imports)]
use examples::{chapter_11, chapter_12, chapter_13, chapter_14};
use light::Light;
use matrix4::Matrix4;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use tuple::Tuple;

use crate::{
    camera::Camera, material::Material, pattern::Pattern, shape::SimpleObject, world::World,
};

fn test_scene() -> (Camera, World) {
    let mut floor = SimpleObject::plane();
    *floor.material_mut() =
        Material::with_pattern(Pattern::checkered(Color::black(), Color::white()));
    floor.material_mut().color = Color::new(1., 0.9, 0.9);
    floor.material_mut().specular = 0.;

    let mut middle = SimpleObject::glass_sphere();
    *middle.transform_mut() = Matrix4::translation(-0.5, 1., 0.5);
    // let mut pattern = Pattern::ring(Color::rgb255(0, 240, 10), Color::rgb255(10, 200, 25));
    // *pattern.transform_mut() = Matrix4::rotation_z(degrees(35.))
    //     * Matrix4::rotation_x(degrees(-60.))
    //     * Matrix4::scaling(0.45, 0.45, 0.45);
    // middle.material_mut().color = Color::new(0.1, 1., 0.5);
    // middle.material_mut().diffuse = 0.7;
    // middle.material_mut().specular = 0.3;
    middle.material_mut().reflective = 1.;

    let mut right = SimpleObject::sphere();
    *right.transform_mut() = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    right.material_mut().color = Color::new(0.5, 1., 0.1);
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;
    right.material_mut().reflective = 1.0;

    let mut left = SimpleObject::sphere();
    *left.transform_mut() =
        Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    left.material_mut().color = Color::new(1., 0.8, 0.1);
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;
    left.material_mut().reflective = 1.0;

    let mut world = World::new();
    world.add_object(floor);
    world.add_object(middle);
    world.add_object(right);
    world.add_object(left);
    world.add_object(examples::back_wall());
    world.add_object(examples::right_wall());

    world.add_light(Light::point_light(
        Tuple::point(-10., 10., -10.),
        Color::white(),
    ));

    let mut camera = Camera::new(WIDTH as i32, HEIGHT as i32, PI / 3.);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 1.5, -5.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}

fn test_scene2(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    // world.add_group(shape::hexagon());
    world.add_light(Light::point_light(
        Tuple::point(-10., 10., -10.),
        Color::white(),
    ));

    let mut camera = Camera::new(width as i32, height as i32, PI / 3.);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 1.5, -5.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}
const WIDTH: usize = 320;
const HEIGHT: usize = 190;

fn main() {
    let (_camera, _world) = chapter_11::scene(WIDTH, HEIGHT);
    let (_camera, _world) = chapter_12::scene(WIDTH, HEIGHT);
    let (_camera, _world) = chapter_13::scene(WIDTH, HEIGHT);
    let (camera, world) = chapter_14::scene(WIDTH, HEIGHT);
    let ppm = camera.render(&world).to_ppm();

    let mut f = File::create("./output.ppm").expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
