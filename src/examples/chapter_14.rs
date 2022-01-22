use crate::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    math::matrix4::Matrix4,
    math::transformations,
    math::tuple::Tuple,
    shape::cone::Cone,
    shape::cylinder::Cylinder,
    shape::{Object, Shape, SimpleObject},
    world::World,
};

/// Scene by Manoël Trapier
/// https://github.com/Godzil/DoRayMe/blob/45f5f8098e50ce746d4d4d130cffea1b9f98174f/tests/ch14_test.cpp

fn leg() -> Object {
    let mut s = Object::sphere();
    s.transform = Matrix4::translation(0., 0., -1.) * Matrix4::scaling(0.25, 0.25, 0.25);

    let mut cylinder = Cylinder::new();
    cylinder.minimum = 0.;
    cylinder.maximum = 1.;
    cylinder.closed = false;
    let mut cylinder = Object::new(Shape::Cylinder(cylinder));
    cylinder.transform = Matrix4::translation(0., 0., -1.)
        * Matrix4::rotation_y(-0.5236)
        * Matrix4::rotation_z(-1.5708)
        * Matrix4::scaling(0.25, 1., 0.25);

    Object::group(vec![s, cylinder])
}

fn cap() -> Object {
    let mut group = Vec::with_capacity(6);

    let mut cone = Cone::new();
    cone.minimum = -1.;
    cone.maximum = 0.;
    cone.closed = false;
    let mut cone = Object::new(Shape::Cone(cone));
    cone.transform = Matrix4::rotation_x(-0.7854) * Matrix4::scaling(0.24606, 1.37002, 0.24606);
    group.push(cone);

    let mut cone = Cone::new();
    cone.minimum = -1.;
    cone.maximum = 0.;
    cone.closed = false;
    let mut cone = Object::new(Shape::Cone(cone));
    cone.transform = Matrix4::rotation_y(1.0472)
        * Matrix4::rotation_x(-0.7854)
        * Matrix4::scaling(0.24606, 1.37002, 0.24606);
    group.push(cone);

    let mut cone = Cone::new();
    cone.minimum = -1.;
    cone.maximum = 0.;
    cone.closed = false;
    let mut cone = Object::new(Shape::Cone(cone));
    cone.transform = Matrix4::rotation_y(2.0944)
        * Matrix4::rotation_x(-0.7854)
        * Matrix4::scaling(0.24606, 1.37002, 0.24606);
    group.push(cone);

    let mut cone = Cone::new();
    cone.minimum = -1.;
    cone.maximum = 0.;
    cone.closed = false;
    let mut cone = Object::new(Shape::Cone(cone));
    cone.transform = Matrix4::rotation_y(3.1416)
        * Matrix4::rotation_x(-0.7854)
        * Matrix4::scaling(0.24606, 1.37002, 0.24606);
    group.push(cone);

    let mut cone = Cone::new();
    cone.minimum = -1.;
    cone.maximum = 0.;
    cone.closed = false;
    let mut cone = Object::new(Shape::Cone(cone));
    cone.transform = Matrix4::rotation_y(4.1888)
        * Matrix4::rotation_x(-0.7854)
        * Matrix4::scaling(0.24606, 1.37002, 0.24606);
    group.push(cone);

    let mut cone = Cone::new();
    cone.minimum = -1.;
    cone.maximum = 0.;
    cone.closed = false;
    let mut cone = Object::new(Shape::Cone(cone));
    cone.transform = Matrix4::rotation_y(5.236)
        * Matrix4::rotation_x(-0.7854)
        * Matrix4::scaling(0.24606, 1.37002, 0.24606);
    group.push(cone);

    Object::group(group)
}

fn wacky() -> Object {
    let mut group = Vec::with_capacity(8);

    let s = leg();
    group.push(s);

    let mut s = leg();
    s.transform = Matrix4::rotation_y(1.0472);
    group.push(s);

    let mut s = leg();
    s.transform = Matrix4::rotation_y(2.0944);
    group.push(s);

    let mut s = leg();
    s.transform = Matrix4::rotation_y(3.1416);
    group.push(s);

    let mut s = leg();
    s.transform = Matrix4::rotation_y(4.1888);
    group.push(s);

    let mut s = leg();
    s.transform = Matrix4::rotation_y(5.236);
    group.push(s);

    let mut s = cap();
    s.transform = Matrix4::translation(0., 1., 0.);
    group.push(s);

    let mut s = cap();
    s.transform = Matrix4::rotation_x(3.1416) * Matrix4::translation(0., 1., 0.);
    group.push(s);

    Object::group(group)
}

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    let light1 = Light::point_light(
        Tuple::point(10000., 10000., -10000.),
        Color::new(0.25, 0.25, 0.25),
    );
    world.add_light(light1);
    let light2 = Light::point_light(
        Tuple::point(-10000., 10000., -10000.),
        Color::new(0.25, 0.25, 0.25),
    );
    world.add_light(light2);
    let light3 = Light::point_light(
        Tuple::point(10000., -10000., -10000.),
        Color::new(0.25, 0.25, 0.25),
    );
    world.add_light(light3);
    let light4 = Light::point_light(
        Tuple::point(-10000., -10000., -10000.),
        Color::new(0.25, 0.25, 0.25),
    );
    world.add_light(light4);

    /* ----------------------------- */

    /* White background */
    let mut p = SimpleObject::plane();
    p.transform = Matrix4::translation(0., 0., 100.) * Matrix4::rotation_x(1.5708);
    p.material.color = Color::white();
    p.material.ambient = 1.;
    p.material.diffuse = 0.;
    p.material.specular = 0.;
    world.add_object(p);

    let mut wacky_object = wacky();
    wacky_object.transform = Matrix4::translation(-2.8, 0., 0.)
        * Matrix4::rotation_x(0.4363)
        * Matrix4::rotation_y(0.1745);
    let mut material = Material::new();
    material.color = Color::new(0.9, 0.2, 0.4);
    material.ambient = 0.2;
    material.diffuse = 0.8;
    material.specular = 0.7;
    material.shininess = 20.;
    wacky_object.set_material(material);
    world.add_group(wacky_object);

    let mut wacky_object = wacky();
    wacky_object.transform = Matrix4::rotation_y(0.1745);
    let mut material = Material::new();
    material.color = Color::new(0.2, 0.9, 0.6);
    material.ambient = 0.2;
    material.diffuse = 0.8;
    material.specular = 0.7;
    material.shininess = 20.;
    wacky_object.set_material(material);
    world.add_group(wacky_object);

    let mut wacky_object = wacky();
    wacky_object.transform = Matrix4::translation(2.8, 0., 0.)
        * Matrix4::rotation_x(-0.4363)
        * Matrix4::rotation_y(-0.1745);
    let mut material = Material::new();
    material.color = Color::new(0.2, 0.3, 1.0);
    material.ambient = 0.2;
    material.diffuse = 0.8;
    material.specular = 0.7;
    material.shininess = 20.;
    wacky_object.set_material(material);
    world.add_group(wacky_object);

    /* ----------------------------- */

    let mut camera = Camera::new(width as i32, height as i32, 0.9);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 0., -9.),
        Tuple::point(0., 0., 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}
