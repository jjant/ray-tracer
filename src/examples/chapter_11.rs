use crate::{
    camera::Camera, color::Color, light::Light, material::Material, matrix4::Matrix4,
    pattern::Pattern, shape::Object, transformations, tuple::Tuple, world::World,
};

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
    let mut floor = Object::plane();
    *floor.transform_mut() = Matrix4::rotation_y(0.31415);
    *floor.material_mut() = Material::with_pattern(Pattern::checkered(
        Color::new(0.35, 0.35, 0.35),
        Color::new(0.65, 0.65, 0.65),
    ));
    floor.material_mut().specular = 0.;
    floor.material_mut().reflective = 0.4;
    world.objects.push(floor);

    let mut ceiling = Object::plane();
    *ceiling.transform_mut() = Matrix4::translation(0., 5., 0.);
    ceiling.material_mut().color = Color::new(0.8, 0.8, 0.8);
    ceiling.material_mut().ambient = 0.3;
    ceiling.material_mut().specular = 0.;
    world.objects.push(ceiling);

    let mut west_wall = Object::plane();
    *west_wall.transform_mut() = Matrix4::translation(-5., 0., 0.)
        * Matrix4::rotation_z(1.5708)
        * Matrix4::rotation_y(1.5708);
    *west_wall.material_mut() = wall_material;
    world.objects.push(west_wall);

    let mut east_wall = Object::plane();
    *east_wall.transform_mut() = Matrix4::translation(5., 0., 0.)
        * Matrix4::rotation_z(1.5708)
        * Matrix4::rotation_y(1.5708);
    *east_wall.material_mut() = wall_material;
    world.objects.push(east_wall);

    let mut north_wall = Object::plane();
    *north_wall.transform_mut() = Matrix4::translation(0., 0., 5.) * Matrix4::rotation_x(1.5708);
    *north_wall.material_mut() = wall_material;
    world.objects.push(north_wall);

    let mut south_wall = Object::plane();
    *south_wall.transform_mut() = Matrix4::translation(0., 0., -5.) * Matrix4::rotation_x(1.5708);
    *south_wall.material_mut() = wall_material;
    world.objects.push(south_wall);

    /* Background balls */
    let mut bg1 = Object::sphere();
    *bg1.transform_mut() = Matrix4::translation(4.6, 0.4, 1.) * Matrix4::scaling(0.4, 0.4, 0.4);
    bg1.material_mut().color = Color::new(0.8, 0.5, 0.3);
    bg1.material_mut().shininess = 50.;
    world.objects.push(bg1);

    let mut bg2 = Object::sphere();
    *bg2.transform_mut() = Matrix4::translation(4.7, 0.3, 0.4) * Matrix4::scaling(0.3, 0.3, 0.3);
    bg2.material_mut().color = Color::new(0.9, 0.4, 0.5);
    bg2.material_mut().shininess = 50.;
    world.objects.push(bg2);

    let mut bg3 = Object::sphere();
    *bg3.transform_mut() = Matrix4::translation(-1., 0.5, 4.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    bg3.material_mut().color = Color::new(0.4, 0.9, 0.6);
    bg3.material_mut().shininess = 50.;
    world.objects.push(bg3);

    let mut bg4 = Object::sphere();
    *bg4.transform_mut() = Matrix4::translation(-1.7, 0.3, 4.7) * Matrix4::scaling(0.3, 0.3, 0.3);
    bg4.material_mut().color = Color::new(0.4, 0.6, 0.9);
    bg4.material_mut().shininess = 50.;
    world.objects.push(bg4);

    /* Foreground balls */
    let mut red_ball = Object::sphere();
    *red_ball.transform_mut() = Matrix4::translation(-0.6, 1., 0.6);
    red_ball.material_mut().color = Color::new(1., 0.3, 0.2);
    red_ball.material_mut().shininess = 5.;
    red_ball.material_mut().specular = 0.4;
    world.objects.push(red_ball);

    let mut blue_glass_ball = Object::sphere();
    *blue_glass_ball.transform_mut() =
        Matrix4::translation(0.6, 0.7, -0.6) * Matrix4::scaling(0.7, 0.7, 0.7);
    blue_glass_ball.material_mut().color = Color::new(0., 0., 0.2);
    blue_glass_ball.material_mut().ambient = 0.;
    blue_glass_ball.material_mut().diffuse = 0.4;
    blue_glass_ball.material_mut().specular = 0.9;
    blue_glass_ball.material_mut().shininess = 300.;
    blue_glass_ball.material_mut().transparency = 0.9;
    blue_glass_ball.material_mut().refractive_index = 1.5;
    world.objects.push(blue_glass_ball);

    let mut green_glass_ball = Object::sphere();
    *green_glass_ball.transform_mut() =
        Matrix4::translation(-0.7, 0.5, -0.8) * Matrix4::scaling(0.5, 0.5, 0.5);
    green_glass_ball.material_mut().color = Color::new(0., 0.2, 0.);
    green_glass_ball.material_mut().ambient = 0.;
    green_glass_ball.material_mut().diffuse = 0.4;
    green_glass_ball.material_mut().specular = 0.9;
    green_glass_ball.material_mut().shininess = 300.;
    green_glass_ball.material_mut().transparency = 0.9;
    green_glass_ball.material_mut().refractive_index = 1.5;
    world.objects.push(green_glass_ball);

    world.light = Some(Light::point_light(
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
