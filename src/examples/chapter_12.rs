use crate::{
    camera::Camera, color::Color, light::Light, material::Material, matrix4::Matrix4,
    pattern::Pattern, shape::Object, transformations, tuple::Tuple, world::World,
};

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    let cube = Object::cube();

    world.objects.push(cube);

    world.light = Some(Light::point_light(
        Tuple::point(-4.9, 4.9, -1.),
        Color::white(),
    ));

    let mut camera = Camera::new(width as i32, height as i32, 1.152);
    camera.transform = transformations::view_transform(
        Tuple::point(-5.6, 3.5, -3.9),
        Tuple::point(-0.6, 1., -0.8),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}
