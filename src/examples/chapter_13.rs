use crate::{
    camera::Camera,
    color::Color,
    light::Light,
    material::Material,
    math::matrix4::Matrix4,
    math::transformations,
    math::tuple::Tuple,
    pattern::Pattern,
    shape::cylinder::Cylinder,
    shape::{Shape, SimpleObject},
    world::World,
};

pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut w = World::new();

    w.add_light(Light::point_light(
        Tuple::point(1., 6.9, -4.9),
        Color::new(1., 1., 1.),
    ));

    /* ----------------------------- */

    let mut floor = SimpleObject::plane();
    *floor.material_mut() = Material::with_pattern(Pattern::checkered(
        Color::new(0.5, 0.5, 0.5),
        Color::new(0.75, 0.75, 0.75),
    ));
    *floor.transform_mut() = Matrix4::rotation_y(0.3) * Matrix4::scaling(0.25, 0.25, 0.25);
    w.add_object(floor);

    /* ----------------------------- */

    let mut cylinder1 = Cylinder::new();
    cylinder1.minimum = 0.;
    cylinder1.maximum = 0.75;
    cylinder1.closed = true;
    let mut cylinder1 = SimpleObject::new(Shape::Cylinder(cylinder1));
    *cylinder1.transform_mut() = Matrix4::translation(-1., 0., 1.) * Matrix4::scaling(0.5, 1., 0.5);
    cylinder1.material_mut().color = Color::new(0., 0., 0.6);
    cylinder1.material_mut().diffuse = 0.1;
    cylinder1.material_mut().specular = 0.9;
    cylinder1.material_mut().shininess = 300.;
    cylinder1.material_mut().reflective = 0.9;
    w.add_object(cylinder1);

    /* ----------------------------- */
    /* Concentrics */
    let mut cons1 = Cylinder::new();
    cons1.minimum = 0.;
    cons1.maximum = 0.2;
    cons1.closed = false;
    let mut cons1 = SimpleObject::new(Shape::Cylinder(cons1));
    *cons1.transform_mut() = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.8, 1., 0.8);
    cons1.material_mut().color = Color::new(1., 1., 0.3);
    cons1.material_mut().ambient = 0.1;
    cons1.material_mut().diffuse = 0.8;
    cons1.material_mut().specular = 0.9;
    cons1.material_mut().shininess = 300.;
    w.add_object(cons1);

    let mut cons2 = Cylinder::new();
    cons2.minimum = 0.;
    cons2.maximum = 0.3;
    cons2.closed = false;
    let mut cons2 = SimpleObject::new(Shape::Cylinder(cons2));
    *cons2.transform_mut() = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.6, 1., 0.6);
    cons2.material_mut().color = Color::new(1., 0.9, 0.4);
    cons2.material_mut().ambient = 0.1;
    cons2.material_mut().diffuse = 0.8;
    cons2.material_mut().specular = 0.9;
    cons2.material_mut().shininess = 300.;
    w.add_object(cons2);

    let mut cons3 = Cylinder::new();
    cons3.minimum = 0.;
    cons3.maximum = 0.4;
    cons3.closed = false;
    let mut cons3 = SimpleObject::new(Shape::Cylinder(cons3));
    *cons3.transform_mut() = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.4, 1., 0.4);
    cons3.material_mut().color = Color::new(1., 0.8, 0.5);
    cons3.material_mut().ambient = 0.1;
    cons3.material_mut().diffuse = 0.8;
    cons3.material_mut().specular = 0.9;
    cons3.material_mut().shininess = 300.;
    w.add_object(cons3);

    let mut cons4 = Cylinder::new();
    cons4.minimum = 0.;
    cons4.maximum = 0.5;
    cons4.closed = true;
    let mut cons4 = SimpleObject::new(Shape::Cylinder(cons4));
    *cons4.transform_mut() = Matrix4::translation(1., 0., 0.) * Matrix4::scaling(0.2, 1., 0.2);
    cons4.material_mut().color = Color::new(1., 0.7, 0.6);
    cons4.material_mut().ambient = 0.1;
    cons4.material_mut().diffuse = 0.8;
    cons4.material_mut().specular = 0.9;
    cons4.material_mut().shininess = 300.;
    w.add_object(cons4);

    /* decoratives cylinders */
    let mut deco1 = Cylinder::new();
    deco1.minimum = 0.;
    deco1.maximum = 0.3;
    deco1.closed = true;
    let mut deco1 = SimpleObject::new(Shape::Cylinder(deco1));
    *deco1.transform_mut() = Matrix4::translation(0., 0., -0.75) * Matrix4::scaling(0.05, 1., 0.05);
    deco1.material_mut().color = Color::new(1., 0., 0.);
    deco1.material_mut().ambient = 0.1;
    deco1.material_mut().diffuse = 0.9;
    deco1.material_mut().specular = 0.9;
    deco1.material_mut().shininess = 300.;
    w.add_object(deco1);

    let mut deco2 = Cylinder::new();
    deco2.minimum = 0.;
    deco2.maximum = 0.3;
    deco2.closed = true;
    let mut deco2 = SimpleObject::new(Shape::Cylinder(deco2));
    *deco2.transform_mut() = Matrix4::translation(0., 0., -2.25)
        * Matrix4::rotation_y(-0.15)
        * Matrix4::translation(0., 0., 1.5)
        * Matrix4::scaling(0.05, 1., 0.05);
    deco2.material_mut().color = Color::new(1., 1., 0.);
    deco2.material_mut().ambient = 0.1;
    deco2.material_mut().diffuse = 0.9;
    deco2.material_mut().specular = 0.9;
    deco2.material_mut().shininess = 300.;
    w.add_object(deco2);

    let mut deco3 = Cylinder::new();
    deco3.minimum = 0.;
    deco3.maximum = 0.3;
    deco3.closed = true;
    let mut deco3 = SimpleObject::new(Shape::Cylinder(deco3));
    *deco3.transform_mut() = Matrix4::translation(0., 0., -2.25)
        * Matrix4::rotation_y(-0.3)
        * Matrix4::translation(0., 0., 1.5)
        * Matrix4::scaling(0.05, 1., 0.05);
    deco3.material_mut().color = Color::new(0., 1., 0.);
    deco3.material_mut().ambient = 0.1;
    deco3.material_mut().diffuse = 0.9;
    deco3.material_mut().specular = 0.9;
    deco3.material_mut().shininess = 300.;
    w.add_object(deco3);

    let mut deco4 = Cylinder::new();
    deco4.minimum = 0.;
    deco4.maximum = 0.3;
    deco4.closed = true;
    let mut deco4 = SimpleObject::new(Shape::Cylinder(deco4));
    *deco4.transform_mut() = Matrix4::translation(0., 0., -2.25)
        * Matrix4::rotation_y(-0.45)
        * Matrix4::translation(0., 0., 1.5)
        * Matrix4::scaling(0.05, 1., 0.05);
    deco4.material_mut().color = Color::new(0., 1., 1.);
    deco4.material_mut().ambient = 0.1;
    deco4.material_mut().diffuse = 0.9;
    deco4.material_mut().specular = 0.9;
    deco4.material_mut().shininess = 300.;
    w.add_object(deco4);

    /* glass cylinder */
    let mut glass_cylinder = Cylinder::new();
    glass_cylinder.minimum = 0.0001;
    glass_cylinder.maximum = 0.5;
    glass_cylinder.closed = true;
    let mut glass_cylinder = SimpleObject::new(Shape::Cylinder(glass_cylinder));

    *glass_cylinder.transform_mut() =
        Matrix4::translation(0., 0., -1.5) * Matrix4::scaling(0.33, 1., 0.33);
    glass_cylinder.material_mut().color = Color::new(0.25, 0., 0.);
    glass_cylinder.material_mut().diffuse = 0.1;
    glass_cylinder.material_mut().specular = 0.9;
    glass_cylinder.material_mut().shininess = 300.;
    glass_cylinder.material_mut().reflective = 0.9;
    glass_cylinder.material_mut().transparency = 0.9;
    glass_cylinder.material_mut().refractive_index = 1.5;
    w.add_object(glass_cylinder);

    let mut camera = Camera::new(width as i32, height as i32, 0.314);
    camera.transform = transformations::view_transform(
        Tuple::point(8., 3.5, -9.),
        Tuple::point(0., 0.3, 0.),
        Tuple::vector(0., 1., 0.),
    );

    (camera, w)
}
