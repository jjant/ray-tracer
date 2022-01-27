use std::f64::consts::PI;

use examples;
use ray_tracer::{
    camera::Camera, color::Color, light::Light, material::Material, math::matrix4::Matrix4,
    math::transformations, math::tuple::Tuple, shape::Object, world::World,
};

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    let mut floor = Object::sphere();
    floor.transform = Matrix4::scaling(10., 0.01, 10.);
    let mut floor_material = Material::new();
    floor_material.casts_shadows = false;
    floor_material.color = Color::new(1., 0.9, 0.9);
    floor_material.specular = 0.;
    floor.set_material(floor_material);
    world.add_object(floor);

    let mut left_wall = Object::sphere();
    left_wall.transform = Matrix4::translation(0., 0., 5.)
        * Matrix4::rotation_y(-PI / 4.)
        * Matrix4::rotation_x(PI / 2.)
        * Matrix4::scaling(10., 0.01, 10.);
    left_wall.set_material(floor_material);
    world.add_object(left_wall);

    let mut right_wall = Object::sphere();
    right_wall.transform = Matrix4::translation(0., 0., 5.)
        * Matrix4::rotation_y(PI / 4.)
        * Matrix4::rotation_x(PI / 2.)
        * Matrix4::scaling(10., 0.01, 10.);
    right_wall.set_material(floor_material);
    world.add_object(right_wall);

    let mut middle = Object::sphere();
    middle.transform = Matrix4::translation(-0.5, 1., 0.5);
    let mut middle_material = Material::new();
    middle_material.casts_shadows = false;
    middle_material.color = Color::new(0.1, 1., 0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.set_material(middle_material);
    world.add_object(middle);

    let mut right = Object::sphere();
    right.transform = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    let mut right_material = Material::new();
    right_material.casts_shadows = false;
    right_material.color = Color::new(0.5, 1., 0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.set_material(right_material);
    world.add_object(right);

    let mut left = Object::sphere();
    left.transform = Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    let mut left_material = Material::new();
    left_material.casts_shadows = false;
    left_material.color = Color::new(1., 0.8, 0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.set_material(left_material);
    world.add_object(left);

    let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::white());
    world.add_light(light);

    let mut camera = Camera::new(width as i32, height as i32, PI / 3.);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 1.5, -5.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

pub fn main() {
    let (camera, world) = scene(WIDTH, HEIGHT);
    examples::run_and_save_scene("chapter_7", camera, world);
}
