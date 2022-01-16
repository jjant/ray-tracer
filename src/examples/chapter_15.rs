use crate::{
    camera::Camera, color::Color, light::Light, shape::Object, transformations, triangle::Triangle,
    tuple::Tuple, world::World,
};

pub fn load_model() -> Object {
    let file_contents = std::fs::read_to_string("teapot.obj").unwrap();
    let obj = Triangle::from_obj_file(&file_contents).unwrap();
    let group = obj.to_group();

    group
}

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    let light1 = Light::point_light(Tuple::point(10000., 10000., -10000.), Color::white());
    world.add_light(light1);

    let group = load_model();
    world.add_group(group);
    /* ----------------------------- */

    let mut camera = Camera::new(width as i32, height as i32, 0.9);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 30., -30.),
        Tuple::point(0., 0., 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}
