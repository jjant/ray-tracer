pub mod chapter_11;

use crate::{
    color::Color, material::Material, matrix4::Matrix4, misc::degrees, pattern::Pattern,
    shape::Object,
};

fn wall_material() -> Material {
    let light_gray = Color::rgb255(20, 20, 20);
    let dark_gray = Color::rgb255(130, 130, 130);

    let mut pattern = Pattern::striped(dark_gray, light_gray);
    *pattern.transform_mut() = Matrix4::rotation_y(degrees(-90.));

    let mut material = Material::with_pattern(pattern);
    material.ambient = 0.7;
    material.diffuse = 0.3;
    material.specular = 0.0;

    material
}

pub fn back_wall() -> Object {
    let mut plane = Object::plane();

    *plane.material_mut() = wall_material();
    *plane.transform_mut() = Matrix4::translation(0., 0., 15.)
        * Matrix4::rotation_y(degrees(-55.0))
        * Matrix4::rotation_x(degrees(90.));

    plane
}

pub fn right_wall() -> Object {
    let mut plane = Object::plane();
    *plane.material_mut() = wall_material();
    *plane.transform_mut() = Matrix4::translation(0., 0., 15.)
        * Matrix4::rotation_y(degrees(-55.0))
        * Matrix4::rotation_x(degrees(90.))
        * Matrix4::rotation_z(degrees(90.));

    plane
}
