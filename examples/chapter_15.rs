use ray_tracer::{
    camera::Camera, color::Color, light::Light, material::Material, math::matrix4::Matrix4,
    math::transformations, math::tuple::Tuple, obj::WavefrontObj, pattern::Pattern, shape::Object,
    world::World,
};
mod misc;
use std::f64::{
    self,
    consts::{FRAC_PI_2, PI},
};

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    let light1 = Light::point_light(Tuple::point(50., 100., 20.), Color::new(0.5, 0.5, 0.5));
    world.add_light(light1);
    let light2 = Light::point_light(Tuple::point(2., 50., 100.), Color::new(0.5, 0.5, 0.5));
    world.add_light(light2);

    /* ----------------------------- */

    let checkered = Pattern::checkered(Color::new(0.35, 0.35, 0.35), Color::new(0.4, 0.4, 0.4));
    let mut planes_material = Material::with_pattern(checkered);
    planes_material.ambient = 1.;
    planes_material.diffuse = 0.;
    planes_material.specular = 0.;

    let mut p = Object::plane();
    let mut p_material = planes_material;
    p_material.reflective = 0.1;
    p.set_material(p_material);
    world.add_group(p);

    let mut p2 = Object::plane();
    p2.transform = Matrix4::translation(0., 0., -10.) * Matrix4::rotation_x(FRAC_PI_2);
    p2.set_material(planes_material);
    world.add_group(p2);

    let mut low_poly_material = Material::new();
    low_poly_material.color = Color::new(1., 0.3, 0.2);
    low_poly_material.shininess = 5.;
    low_poly_material.specular = 0.4;

    let mut teapot = WavefrontObj::from_file("./resources/teapot-low.obj").unwrap();
    teapot.transform = Matrix4::translation(7., 0., 3.)
        * Matrix4::rotation_y(PI * 23. / 22.)
        * Matrix4::rotation_x(-PI / 2.)
        * Matrix4::scaling(0.3, 0.3, 0.3);
    teapot.set_material(low_poly_material);
    world.add_group(teapot);

    let mut teapot2 = WavefrontObj::from_file("./resources/teapot-lowtri.obj").unwrap();
    teapot2.transform = Matrix4::translation(-7., 0., 3.)
        * Matrix4::rotation_y(-PI * 46. / 22.)
        * Matrix4::rotation_x(-PI / 2.)
        * Matrix4::scaling(0.3, 0.3, 0.3);
    teapot2.set_material(low_poly_material);
    world.add_group(teapot2);

    let mut high_poly_material = Material::new();
    high_poly_material.color = Color::new(0.3, 1., 0.2);
    high_poly_material.shininess = 5.;
    high_poly_material.specular = 0.4;
    high_poly_material.reflective = 0.5;

    let mut teapot3 = WavefrontObj::from_file("./resources/teapot.obj").unwrap();
    teapot3.transform = Matrix4::translation(0., 0., -5.)
        * Matrix4::rotation_y(-PI)
        * Matrix4::rotation_x(-PI / 2.)
        * Matrix4::scaling(0.4, 0.4, 0.4);
    teapot3.set_material(high_poly_material);
    world.add_group(teapot3);

    let mut camera = Camera::new(width as i32, height as i32, FRAC_PI_2);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 7., 13.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

fn main() {
    let (camera, world) = scene(WIDTH, HEIGHT);
    misc::run_and_save_scene(module_path!(), camera, world);
}
