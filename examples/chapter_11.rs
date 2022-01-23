use ray_tracer::{
    camera::Camera, color::Color, light::Light, material::Material, math::matrix4::Matrix4,
    math::transformations, math::tuple::Tuple, pattern::Pattern, shape::SimpleObject, world::World,
};
mod misc;

/// Scene by Manoël Trapier
/// https://github.com/Godzil/DoRayMe/blob/45f5f8098e50ce746d4d4d130cffea1b9f98174f/tests/ch11_test.cpp
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
    let mut floor = SimpleObject::plane();
    *floor.transform_mut() = Matrix4::rotation_y(0.31415);
    *floor.material_mut() = Material::with_pattern(Pattern::checkered(
        Color::new(0.35, 0.35, 0.35),
        Color::new(0.65, 0.65, 0.65),
    ));
    floor.material_mut().specular = 0.;
    floor.material_mut().reflective = 0.4;
    world.add_object(floor);

    let mut ceiling = SimpleObject::plane();
    *ceiling.transform_mut() = Matrix4::translation(0., 5., 0.);
    ceiling.material_mut().color = Color::new(0.8, 0.8, 0.8);
    ceiling.material_mut().ambient = 0.3;
    ceiling.material_mut().specular = 0.;
    world.add_object(ceiling);

    let mut west_wall = SimpleObject::plane();
    *west_wall.transform_mut() = Matrix4::translation(-5., 0., 0.)
        * Matrix4::rotation_z(1.5708)
        * Matrix4::rotation_y(1.5708);
    *west_wall.material_mut() = wall_material;
    world.add_object(west_wall);

    let mut east_wall = SimpleObject::plane();
    *east_wall.transform_mut() = Matrix4::translation(5., 0., 0.)
        * Matrix4::rotation_z(1.5708)
        * Matrix4::rotation_y(1.5708);
    *east_wall.material_mut() = wall_material;
    world.add_object(east_wall);

    let mut north_wall = SimpleObject::plane();
    *north_wall.transform_mut() = Matrix4::translation(0., 0., 5.) * Matrix4::rotation_x(1.5708);
    *north_wall.material_mut() = wall_material;
    world.add_object(north_wall);

    let mut south_wall = SimpleObject::plane();
    *south_wall.transform_mut() = Matrix4::translation(0., 0., -5.) * Matrix4::rotation_x(1.5708);
    *south_wall.material_mut() = wall_material;
    world.add_object(south_wall);

    /* Background balls */
    let mut bg1 = SimpleObject::sphere();
    *bg1.transform_mut() = Matrix4::translation(4.6, 0.4, 1.) * Matrix4::scaling(0.4, 0.4, 0.4);
    bg1.material_mut().color = Color::new(0.8, 0.5, 0.3);
    bg1.material_mut().shininess = 50.;
    world.add_object(bg1);

    let mut bg2 = SimpleObject::sphere();
    *bg2.transform_mut() = Matrix4::translation(4.7, 0.3, 0.4) * Matrix4::scaling(0.3, 0.3, 0.3);
    bg2.material_mut().color = Color::new(0.9, 0.4, 0.5);
    bg2.material_mut().shininess = 50.;
    world.add_object(bg2);

    let mut bg3 = SimpleObject::sphere();
    *bg3.transform_mut() = Matrix4::translation(-1., 0.5, 4.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    bg3.material_mut().color = Color::new(0.4, 0.9, 0.6);
    bg3.material_mut().shininess = 50.;
    world.add_object(bg3);

    let mut bg4 = SimpleObject::sphere();
    *bg4.transform_mut() = Matrix4::translation(-1.7, 0.3, 4.7) * Matrix4::scaling(0.3, 0.3, 0.3);
    bg4.material_mut().color = Color::new(0.4, 0.6, 0.9);
    bg4.material_mut().shininess = 50.;
    world.add_object(bg4);

    /* Foreground balls */
    let mut red_ball = SimpleObject::sphere();
    *red_ball.transform_mut() = Matrix4::translation(-0.6, 1., 0.6);
    red_ball.material_mut().color = Color::new(1., 0.3, 0.2);
    red_ball.material_mut().shininess = 5.;
    red_ball.material_mut().specular = 0.4;
    world.add_object(red_ball);

    let mut blue_glass_ball = SimpleObject::sphere();
    *blue_glass_ball.transform_mut() =
        Matrix4::translation(0.6, 0.7, -0.6) * Matrix4::scaling(0.7, 0.7, 0.7);
    blue_glass_ball.material_mut().color = Color::new(0., 0., 0.2);
    blue_glass_ball.material_mut().ambient = 0.;
    blue_glass_ball.material_mut().diffuse = 0.4;
    blue_glass_ball.material_mut().specular = 0.9;
    blue_glass_ball.material_mut().shininess = 300.;
    blue_glass_ball.material_mut().transparency = 0.9;
    blue_glass_ball.material_mut().refractive_index = 1.5;
    world.add_object(blue_glass_ball);

    let mut green_glass_ball = SimpleObject::sphere();
    *green_glass_ball.transform_mut() =
        Matrix4::translation(-0.7, 0.5, -0.8) * Matrix4::scaling(0.5, 0.5, 0.5);
    green_glass_ball.material_mut().color = Color::new(0., 0.2, 0.);
    green_glass_ball.material_mut().ambient = 0.;
    green_glass_ball.material_mut().diffuse = 0.4;
    green_glass_ball.material_mut().specular = 0.9;
    green_glass_ball.material_mut().shininess = 300.;
    green_glass_ball.material_mut().transparency = 0.9;
    green_glass_ball.material_mut().refractive_index = 1.5;
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

fn main() {
    let (camera, world) = scene(WIDTH, HEIGHT);
    misc::run_and_save_scene(module_path!(), camera, world);
}
