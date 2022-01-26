use examples;
use ray_tracer::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    math::matrix4::Matrix4,
    math::transformations,
    math::tuple::Tuple,
    pattern::Pattern,
    shape::{cylinder::Cylinder, Object, Shape},
    world::World,
};
use std::f64::{
    self,
    consts::{FRAC_PI_2, PI},
};

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    /* Add lights */
    let light1 = Light::point_light(Tuple::point(6., 10., 10.), Color::new(0.5, 0.4, 0.5));
    world.add_light(light1);

    /* Add lights */
    let light2 = Light::point_light(Tuple::point(6., 10., -2.5), Color::new(0.5, 0.6, 0.5));
    world.add_light(light2);

    /* ----------------------------- */

    /* Floor */
    let mut p = Object::plane();
    let checkered = Pattern::checkered(Color::new(0.35, 0.35, 0.35), Color::new(0.4, 0.4, 0.4));
    let mut material = Material::with_pattern(checkered);
    material.ambient = 0.2;
    material.diffuse = 1.;
    material.specular = 0.;
    material.reflective = 0.1;
    p.set_material(material);
    p.transform = Matrix4::translation(0., 0., 0.);
    world.add_object(p);

    let mut p2 = Object::plane();
    let mut material = Material::with_pattern(checkered);
    material.ambient = 0.2;
    material.diffuse = 1.;
    material.specular = 0.;
    p2.set_material(material);
    p2.transform = Matrix4::translation(0., 0., -3.) * Matrix4::rotation_x(FRAC_PI_2);
    world.add_object(p2);

    /* ----------------------------- */

    let mut c1 = Cylinder::new();
    c1.minimum = -2.;
    c1.maximum = 2.;
    c1.closed = true;
    let mut c1 = Object::new(Shape::Cylinder(c1));
    let mut material = Material::new();
    material.color = Color::new(1., 0., 0.);
    c1.set_material(material);
    c1.transform = Matrix4::scaling(0.4, 1., 0.4);

    let mut c2 = Cylinder::new();
    c2.minimum = -2.;
    c2.maximum = 2.;
    c2.closed = true;
    let mut c2 = Object::new(Shape::Cylinder(c2));
    let mut material = Material::new();
    material.color = Color::new(0., 1., 0.);
    c2.set_material(material);
    c2.transform = Matrix4::rotation_x(FRAC_PI_2) * Matrix4::scaling(0.4, 1., 0.4);

    let leaf1 = Object::union(c1, c2);

    let mut c3 = Cylinder::new();
    c3.minimum = -2.;
    c3.maximum = 2.;
    c3.closed = true;
    let mut c3 = Object::new(Shape::Cylinder(c3));
    let mut material = Material::new();
    material.color = Color::new(0., 0., 1.);
    c3.set_material(material);
    c3.transform = Matrix4::rotation_z(FRAC_PI_2) * Matrix4::scaling(0.4, 1., 0.4);

    let leaf2 = Object::union(leaf1, c3);

    let mut cb = Object::cube();
    let mut material = Material::new();
    material.reflective = 0.5;
    material.color = Color::new(0.3, 0.3, 0.3);
    material.ambient = 0.;
    material.diffuse = 0.3;
    material.specular = 0.3;
    material.shininess = 20.;
    cb.set_material(material);

    let mut sp = Object::sphere();
    sp.transform = Matrix4::scaling(1.35, 1.35, 1.35);
    let mut material = Material::new();
    material.color = Color::new(0., 0., 0.);
    material.ambient = 0.;
    material.specular = 0.3;
    material.shininess = 20.;
    material.reflective = 0.05;
    material.diffuse = 0.3;
    sp.set_material(material);
    let leaf3 = Object::intersection(sp, cb);

    let mut leaf4 = Object::difference(leaf3, leaf2);
    leaf4.transform = Matrix4::translation(0., 1., 0.8) * Matrix4::rotation_y(-0.45);
    world.add_object(leaf4);

    /* ----------------------------- */

    /* Tricylinder weirdy */
    let mut sp1 = Cylinder::new();
    sp1.minimum = -2.;
    sp1.maximum = 2.;
    sp1.closed = true;
    let mut sp1 = Object::new(Shape::Cylinder(sp1));
    let mut material = Material::new();
    material.color = Color::new(1., 0., 0.);
    sp1.set_material(material);

    let mut sp2 = Cylinder::new();
    sp2.minimum = -2.;
    sp2.maximum = 2.;
    sp2.closed = true;
    let mut sp2 = Object::new(Shape::Cylinder(sp2));
    sp2.transform = Matrix4::rotation_x(FRAC_PI_2);
    let mut material = Material::new();
    material.color = Color::new(0., 1., 0.);
    sp2.set_material(material);

    let mut sp3 = Cylinder::new();
    sp3.minimum = -2.;
    sp3.maximum = 2.;
    sp3.closed = true;
    let mut sp3 = Object::new(Shape::Cylinder(sp3));
    sp3.transform = Matrix4::rotation_z(FRAC_PI_2);
    let mut material = Material::new();
    material.color = Color::new(0., 0., 1.);
    sp3.set_material(material);

    let spleaf1 = Object::intersection(sp1, sp2);
    let mut spleaf2 = Object::intersection(spleaf1, sp3);

    spleaf2.transform = Matrix4::translation(4., 1., -0.1) * Matrix4::rotation_y(0.35);
    world.add_object(spleaf2);

    /* ----------------------------- */

    let mut group = vec![];
    let slice_num = 12;
    for i in 0..slice_num {
        let mut c = Object::cube();
        c.transform = Matrix4::rotation_y((2. * PI / slice_num as f64) * i as f64)
            * Matrix4::scaling(0.1, 1.1, 0.7)
            * Matrix4::translation(0., 0., 0.9);
        let mut material = Material::new();
        material.casts_shadows = false;
        c.set_material(material);

        group.push(c);
    }
    let mut group = Object::group(group);

    let mut material = Material::new();
    material.casts_shadows = false;
    material.ambient = 0.;
    material.diffuse = 0.1;
    material.specular = 0.;
    material.transparency = 1.;
    material.reflective = 1.;
    material.refractive_index = 1.;
    group.set_material(material);

    let mut ball_sp = Object::sphere();
    let mut material = Material::new();
    material.color = Color::new(0.7, 0.2, 0.1);
    ball_sp.set_material(material);

    let mut ball_leaf = Object::intersection(group, ball_sp);
    ball_leaf.transform =
        Matrix4::translation(-4., 1., -0.1) * Matrix4::rotation_y(-0.35) * Matrix4::rotation_z(0.1);

    world.add_object(ball_leaf);

    /* ----------------------------- */
    let mut camera = Camera::new(width as i32, height as i32, FRAC_PI_2);
    camera.transform = transformations::view_transform(
        Tuple::point(0., 3., 5.),
        Tuple::point(0., 1., 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, world)
}

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 600;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

fn main() {
    let (camera, world) = scene(WIDTH, HEIGHT);
    examples::run_and_save_scene(module_path!(), camera, world);
}
