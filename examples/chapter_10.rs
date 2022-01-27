use std::f64::consts::PI;

use examples;
use ray_tracer::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    math::tuple::Tuple,
    math::{matrix4::Matrix4, transformations},
    pattern::Pattern,
    shape::Object,
    world::World,
};

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    let light = Light::point_light(Tuple::point(-10., 10., -10.), Color::white());
    world.add_light(light);

    let mut floor = Object::plane();
    let mut floor_material = Material::with_pattern(Pattern::ring(
        Color::new(1., 0.9, 0.9),
        Color::new(1., 0.2, 0.2),
    ));
    floor_material.specular = 0.;
    floor.set_material(floor_material);
    world.add_object(floor);

    let mut wall = Object::plane();
    let mut wall_pattern = Pattern::striped(Color::new(1., 0.9, 0.9), Color::new(1., 0.2, 0.2));
    wall_pattern.transform = Matrix4::translation(0., 0., 1.) * Matrix4::rotation_y(PI / 4.);
    let mut wall_material = Material::with_pattern(wall_pattern);
    wall_material.specular = 0.;
    wall.transform = Matrix4::translation(0., 0., 5.) * Matrix4::rotation_x(PI / 2.);
    wall.set_material(wall_material);
    world.add_object(wall);

    let mut middle = Object::sphere();
    middle.transform = Matrix4::translation(-0.7, 1., 0.6);
    let mut middle_pattern = Pattern::striped(Color::new(0.1, 1., 0.5), Color::new(0., 0.2, 0.2));
    middle_pattern.transform = Matrix4::rotation_z(PI / 4.)
        * Matrix4::rotation_y(PI / 5.)
        * Matrix4::scaling(0.2, 0.2, 0.2);
    let mut middle_material = Material::with_pattern(middle_pattern);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.set_material(middle_material);
    world.add_object(middle);

    let mut right = Object::sphere();
    right.transform = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    let mut right_pattern = Pattern::striped(Color::new(0.5, 1., 0.1), Color::black());
    right_pattern.transform = Matrix4::scaling(0.1, 0.1, 0.1);
    let mut right_material = Material::with_pattern(right_pattern);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.set_material(right_material);
    world.add_object(right);

    let mut left = Object::sphere();
    left.transform = Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    let mut left_pattern = Pattern::gradient(Color::new(1., 0.8, 0.1), Color::new(0.1, 0.1, 1.));
    left_pattern.transform = Matrix4::translation(1.5, 0., 0.)
        * Matrix4::scaling(2.1, 2., 2.)
        * Matrix4::rotation_y(-PI / 4.);
    let mut left_material = Material::with_pattern(left_pattern);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.set_material(left_material);
    world.add_object(left);

    let mut fourth = Object::sphere();
    fourth.transform = Matrix4::translation(0.5, 0.25, 0.4) * Matrix4::scaling(0.3, 0.3, 0.3);
    let mut fourth_pattern =
        Pattern::checkered(Color::new(0.1, 0.8, 0.1), Color::new(0.8, 1., 0.8));
    fourth_pattern.transform = Matrix4::scaling(0.2, 0.2, 0.2);
    let mut fourth_material = Material::with_pattern(fourth_pattern);
    fourth_material.diffuse = 0.7;
    fourth_material.specular = 0.3;
    fourth.set_material(fourth_material);
    world.add_object(fourth);

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
    examples::run_and_save_scene("chapter_10", camera, world);
}
