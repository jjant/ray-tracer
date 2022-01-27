use examples;
use ray_tracer::{
    camera::Camera, color::Color, light::Light, material::Material, math::matrix4::Matrix4,
    math::transformations, math::tuple::Tuple, pattern::Pattern, shape::Object, world::World,
};

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    let wall_material = {
        let mut pattern =
            Pattern::striped(Color::new(0.45, 0.45, 0.45), Color::new(0.55, 0.55, 0.55));
        *pattern.transform_mut() = Matrix4::scaling(0.25, 0.25, 0.25) * Matrix4::rotation_y(1.5708);

        let mut material = Material::with_pattern(pattern);
        material.ambient = 0.;
        material.diffuse = 0.4;
        material.specular = 0.;
        material.reflective = 0.3;

        material
    };

    /* Walls */
    let mut floor = Object::plane();
    floor.transform = Matrix4::rotation_y(0.31415);
    let mut material = Material::with_pattern(Pattern::checkered(
        Color::new(0.35, 0.35, 0.35),
        Color::new(0.65, 0.65, 0.65),
    ));
    material.specular = 0.;
    material.reflective = 0.4;
    floor.set_material(material);
    world.add_object(floor);

    let mut ceiling = Object::plane();
    ceiling.transform = Matrix4::translation(0., 5., 0.);
    let mut material = Material::new();
    material.color = Color::new(0.8, 0.8, 0.8);
    material.ambient = 0.3;
    material.specular = 0.;
    ceiling.set_material(material);
    world.add_object(ceiling);

    let mut west_wall = Object::plane();
    west_wall.transform = Matrix4::translation(-5., 0., 0.)
        * Matrix4::rotation_z(1.5708)
        * Matrix4::rotation_y(1.5708);
    material = wall_material;
    west_wall.set_material(material);
    world.add_object(west_wall);

    let mut east_wall = Object::plane();
    east_wall.transform = Matrix4::translation(5., 0., 0.)
        * Matrix4::rotation_z(1.5708)
        * Matrix4::rotation_y(1.5708);
    material = wall_material;
    east_wall.set_material(material);
    world.add_object(east_wall);

    let mut north_wall = Object::plane();
    north_wall.transform = Matrix4::translation(0., 0., 5.) * Matrix4::rotation_x(1.5708);
    material = wall_material;
    north_wall.set_material(material);
    world.add_object(north_wall);

    let mut south_wall = Object::plane();
    south_wall.transform = Matrix4::translation(0., 0., -5.) * Matrix4::rotation_x(1.5708);
    material = wall_material;
    south_wall.set_material(material);
    world.add_object(south_wall);

    /* Background balls */
    let mut bg1 = Object::sphere();
    bg1.transform = Matrix4::translation(4.6, 0.4, 1.) * Matrix4::scaling(0.4, 0.4, 0.4);
    let mut material = Material::new();
    material.color = Color::new(0.8, 0.5, 0.3);
    material.shininess = 50.;
    bg1.set_material(material);
    world.add_object(bg1);

    let mut bg2 = Object::sphere();
    bg2.transform = Matrix4::translation(4.7, 0.3, 0.4) * Matrix4::scaling(0.3, 0.3, 0.3);
    let mut material = Material::new();
    material.color = Color::new(0.9, 0.4, 0.5);
    material.shininess = 50.;
    bg2.set_material(material);
    world.add_object(bg2);

    let mut bg3 = Object::sphere();
    bg3.transform = Matrix4::translation(-1., 0.5, 4.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    material.color = Color::new(0.4, 0.9, 0.6);
    material.shininess = 50.;
    bg3.set_material(material);
    world.add_object(bg3);

    let mut bg4 = Object::sphere();
    bg4.transform = Matrix4::translation(-1.7, 0.3, 4.7) * Matrix4::scaling(0.3, 0.3, 0.3);
    let mut material = Material::new();
    material.color = Color::new(0.4, 0.6, 0.9);
    material.shininess = 50.;
    bg4.set_material(material);
    world.add_object(bg4);

    /* Foreground balls */
    let mut red_ball = Object::sphere();
    red_ball.transform = Matrix4::translation(-0.6, 1., 0.6);
    let mut material = Material::new();
    material.color = Color::new(1., 0.3, 0.2);
    material.shininess = 5.;
    material.specular = 0.4;
    red_ball.set_material(material);
    world.add_object(red_ball);

    let mut blue_glass_ball = Object::sphere();
    blue_glass_ball.transform =
        Matrix4::translation(0.6, 0.7, -0.6) * Matrix4::scaling(0.7, 0.7, 0.7);
    let mut material = Material::new();
    material.color = Color::new(0., 0., 0.2);
    material.ambient = 0.;
    material.diffuse = 0.4;
    material.specular = 0.9;
    material.shininess = 300.;
    material.transparency = 0.9;
    material.refractive_index = 1.5;
    blue_glass_ball.set_material(material);
    world.add_object(blue_glass_ball);

    let mut material = Material::new();
    material.color = Color::new(0., 0.2, 0.);
    material.ambient = 0.;
    material.diffuse = 0.4;
    material.specular = 0.9;
    material.shininess = 300.;
    material.transparency = 0.9;
    material.refractive_index = 1.5;
    let mut green_glass_ball = Object::sphere();
    green_glass_ball.transform =
        Matrix4::translation(-0.7, 0.5, -0.8) * Matrix4::scaling(0.5, 0.5, 0.5);
    green_glass_ball.set_material(material);
    world.add_object(green_glass_ball);

    world.add_light(Light::point_light(
        Tuple::point(-4.9, 4.9, -1.),
        Color::white(),
    ));

    let mut camera = Camera::new(width as i32, height as i32, 1.152);
    camera.transform = transformations::view_transform(
        Tuple::point(-2.6, 1.5, -3.9),
        Tuple::point(-0.6, 1., -0.8),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

pub fn main() {
    let (camera, world) = scene(WIDTH, HEIGHT);
    examples::run_and_save_scene("chapter_11", camera, world);
}
