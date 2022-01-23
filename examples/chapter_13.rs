use ray_tracer::{
    camera::Camera, color::Color, light::Light, material::Material, math::matrix4::Matrix4,
    math::transformations, math::tuple::Tuple, pattern::Pattern, shape::cylinder::Cylinder,
    shape::Object, shape::Shape, world::World,
};
mod misc;

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut w = World::new();

    w.add_light(Light::point_light(
        Tuple::point(1., 6.9, -4.9),
        Color::new(1., 1., 1.),
    ));

    /* ----------------------------- */

    let mut floor = Object::plane();
    let material = Material::with_pattern(Pattern::checkered(
        Color::new(0.5, 0.5, 0.5),
        Color::new(0.75, 0.75, 0.75),
    ));
    floor.set_material(material);
    floor.transform = Matrix4::rotation_y(0.3) * Matrix4::scaling(0.25, 0.25, 0.25);
    w.add_object(floor);

    /* ----------------------------- */

    let mut cylinder1 = Cylinder::new();
    cylinder1.minimum = 0.;
    cylinder1.maximum = 0.75;
    cylinder1.closed = true;
    let mut cylinder1 = Object::new(Shape::Cylinder(cylinder1));
    cylinder1.transform = Matrix4::translation(-1., 0., 1.) * Matrix4::scaling(0.5, 1., 0.5);
    let mut material = Material::new();
    material.color = Color::new(0., 0., 0.6);
    material.diffuse = 0.1;
    material.specular = 0.9;
    material.shininess = 300.;
    material.reflective = 0.9;
    cylinder1.set_material(material);
    w.add_object(cylinder1);

    /* ----------------------------- */
    /* Concentrics */
    let mut cons1 = Cylinder::new();
    cons1.minimum = 0.;
    cons1.maximum = 0.2;
    cons1.closed = false;
    let mut cons1 = Object::new(Shape::Cylinder(cons1));
    cons1.transform = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.8, 1., 0.8);
    let mut material = Material::new();
    material.color = Color::new(1., 1., 0.3);
    material.ambient = 0.1;
    material.diffuse = 0.8;
    material.specular = 0.9;
    material.shininess = 300.;
    cons1.set_material(material);
    w.add_object(cons1);

    let mut cons2 = Cylinder::new();
    cons2.minimum = 0.;
    cons2.maximum = 0.3;
    cons2.closed = false;
    let shape = Shape::Cylinder(cons2);
    let mut cons2 = Object::new(shape);
    cons2.transform = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.6, 1., 0.6);
    let mut material = Material::new();
    material.color = Color::new(1., 0.9, 0.4);
    material.ambient = 0.1;
    material.diffuse = 0.8;
    material.specular = 0.9;
    material.shininess = 300.;
    cons2.set_material(material);
    w.add_object(cons2);

    let mut cons3 = Cylinder::new();
    cons3.minimum = 0.;
    cons3.maximum = 0.4;
    cons3.closed = false;
    let shape = Shape::Cylinder(cons3);
    let mut cons3 = Object::new(shape);
    cons3.transform = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.4, 1., 0.4);
    let mut material = Material::new();
    material.color = Color::new(1., 0.8, 0.5);
    material.ambient = 0.1;
    material.diffuse = 0.8;
    material.specular = 0.9;
    material.shininess = 300.;
    cons3.set_material(material);
    w.add_object(cons3);

    let mut cons4 = Cylinder::new();
    cons4.minimum = 0.;
    cons4.maximum = 0.5;
    cons4.closed = true;
    let shape = Shape::Cylinder(cons4);
    let mut cons4 = Object::new(shape);
    cons4.transform = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.2, 1., 0.2);
    let mut material = Material::new();
    material.color = Color::new(1., 0.7, 0.6);
    material.ambient = 0.1;
    material.diffuse = 0.8;
    material.specular = 0.9;
    material.shininess = 300.;
    cons4.set_material(material);
    w.add_object(cons4);

    /* decoratives cylinders */
    let mut deco1 = Cylinder::new();
    deco1.minimum = 0.;
    deco1.maximum = 0.3;
    deco1.closed = true;
    let shape = Shape::Cylinder(deco1);
    let mut deco1 = Object::new(shape);
    deco1.transform = Matrix4::translation(0., 0., -0.75) * Matrix4::scaling(0.05, 1., 0.05);
    material.color = Color::new(1., 0., 0.);
    material.ambient = 0.1;
    material.diffuse = 0.9;
    material.specular = 0.9;
    material.shininess = 300.;
    deco1.set_material(material);
    w.add_object(deco1);

    let mut deco2 = Cylinder::new();
    deco2.minimum = 0.;
    deco2.maximum = 0.3;
    deco2.closed = true;
    let shape = Shape::Cylinder(deco2);
    let mut deco2 = Object::new(shape);
    deco2.transform = Matrix4::translation(0., 0., -2.25)
        * Matrix4::rotation_y(-0.15)
        * Matrix4::translation(0., 0., 1.5)
        * Matrix4::scaling(0.05, 1., 0.05);
    material.color = Color::new(1., 1., 0.);
    material.ambient = 0.1;
    material.diffuse = 0.9;
    material.specular = 0.9;
    material.shininess = 300.;
    deco2.set_material(material);
    w.add_object(deco2);

    let mut deco3 = Cylinder::new();
    deco3.minimum = 0.;
    deco3.maximum = 0.3;
    deco3.closed = true;
    let shape = Shape::Cylinder(deco3);
    let mut deco3 = Object::new(shape);
    deco3.transform = Matrix4::translation(0., 0., -2.25)
        * Matrix4::rotation_y(-0.3)
        * Matrix4::translation(0., 0., 1.5)
        * Matrix4::scaling(0.05, 1., 0.05);
    material.color = Color::new(0., 1., 0.);
    material.ambient = 0.1;
    material.diffuse = 0.9;
    material.specular = 0.9;
    material.shininess = 300.;
    deco3.set_material(material);
    w.add_object(deco3);

    let mut deco4 = Cylinder::new();
    deco4.minimum = 0.;
    deco4.maximum = 0.3;
    deco4.closed = true;
    let shape = Shape::Cylinder(deco4);
    let mut deco4 = Object::new(shape);
    deco4.transform = Matrix4::translation(0., 0., -2.25)
        * Matrix4::rotation_y(-0.45)
        * Matrix4::translation(0., 0., 1.5)
        * Matrix4::scaling(0.05, 1., 0.05);
    material.color = Color::new(0., 1., 1.);
    material.ambient = 0.1;
    material.diffuse = 0.9;
    material.specular = 0.9;
    material.shininess = 300.;
    deco4.set_material(material);
    w.add_object(deco4);

    /* glass cylinder */
    let mut glass_cylinder = Cylinder::new();
    glass_cylinder.minimum = 0.0001;
    glass_cylinder.maximum = 0.5;
    glass_cylinder.closed = true;
    let shape = Shape::Cylinder(glass_cylinder);
    let mut glass_cylinder = Object::new(shape);
    glass_cylinder.transform =
        Matrix4::translation(0., 0., -1.5) * Matrix4::scaling(0.33, 1., 0.33);
    let mut material = Material::new();
    material.color = Color::new(0.25, 0., 0.);
    material.diffuse = 0.1;
    material.specular = 0.9;
    material.shininess = 300.;
    material.reflective = 0.9;
    material.transparency = 0.9;
    material.refractive_index = 1.5;
    glass_cylinder.set_material(material);
    w.add_object(glass_cylinder);

    let mut camera = Camera::new(width as i32, height as i32, 0.314);
    camera.transform = transformations::view_transform(
        Tuple::point(8., 3.5, -9.),
        Tuple::point(0., 0.3, 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, w)
}

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

fn main() {
    let (camera, world) = scene(WIDTH, HEIGHT);
    misc::run_and_save_scene(module_path!(), camera, world);
}
