use ray_tracer::{
    camera::Camera, color::Color, light::Light, material::Material, math::matrix4::Matrix4,
    math::transformations, math::tuple::Tuple, pattern::Pattern, shape::SimpleObject, world::World,
};
mod misc;

/// Scene by Manoël Trapier
/// https://github.com/Godzil/DoRayMe/blob/45f5f8098e50ce746d4d4d130cffea1b9f98174f/tests/ch12_test.cpp
pub fn scene(width: usize, height: usize) -> (Camera, World) {
    let mut world = World::new();

    world.add_light(Light::point_light(
        Tuple::point(0., 6.9, -5.),
        Color::new(1., 1., 0.9),
    ));

    /* The floor / ceiling */
    let mut floor = SimpleObject::cube();
    *floor.transform_mut() = Matrix4::scaling(20., 7., 20.) * Matrix4::translation(0., 1., 0.);
    let mut pattern = Pattern::checkered(Color::new(0., 0., 0.), Color::new(0.25, 0.25, 0.25));
    *pattern.transform_mut() = Matrix4::scaling(0.07, 0.07, 0.07);

    *floor.material_mut() = Material::with_pattern(pattern);
    floor.material_mut().ambient = 0.25;
    floor.material_mut().diffuse = 0.7;
    floor.material_mut().specular = 0.9;
    floor.material_mut().shininess = 300.;
    floor.material_mut().reflective = 0.1;
    world.add_object(floor);

    /* Walls */
    let mut walls = SimpleObject::cube();
    *walls.transform_mut() = Matrix4::scaling(10., 10., 10.);
    let mut pattern = Pattern::checkered(
        Color::new(0.4863, 0.3765, 0.2941),
        Color::new(0.3725, 0.2902, 0.2275),
    );
    *pattern.transform_mut() = Matrix4::scaling(0.05, 20., 0.05);
    *walls.material_mut() = Material::with_pattern(pattern);

    walls.material_mut().ambient = 0.1;
    walls.material_mut().diffuse = 0.7;
    walls.material_mut().specular = 0.9;
    walls.material_mut().shininess = 300.;
    walls.material_mut().reflective = 0.1;
    world.add_object(walls);

    /* Table top */
    let mut table_top = SimpleObject::cube();
    *table_top.transform_mut() = Matrix4::translation(0., 3.1, 0.) * Matrix4::scaling(3., 0.1, 2.);
    let mut pattern = Pattern::striped(
        Color::new(0.5529, 0.4235, 0.3255),
        Color::new(0.6588, 0.5098, 0.4000),
    );
    *pattern.transform_mut() = Matrix4::scaling(0.05, 0.05, 0.05) * Matrix4::rotation_y(0.1);
    *table_top.material_mut() = Material::with_pattern(pattern);
    table_top.material_mut().ambient = 0.1;
    table_top.material_mut().diffuse = 0.7;
    table_top.material_mut().specular = 0.9;
    table_top.material_mut().shininess = 300.;
    table_top.material_mut().reflective = 0.2;
    world.add_object(table_top);

    /* Leg 1 */
    let mut leg1 = SimpleObject::cube();
    *leg1.transform_mut() = Matrix4::translation(2.7, 1.5, -1.7) * Matrix4::scaling(0.1, 1.5, 0.1);
    leg1.material_mut().color = Color::new(0.5529, 0.4235, 0.3255);
    leg1.material_mut().ambient = 0.2;
    leg1.material_mut().diffuse = 0.7;
    world.add_object(leg1);

    /* Leg 2 */
    let mut leg2 = SimpleObject::cube();
    *leg2.transform_mut() = Matrix4::translation(2.7, 1.5, 1.7) * Matrix4::scaling(0.1, 1.5, 0.1);
    leg2.material_mut().color = Color::new(0.5529, 0.4235, 0.3255);
    leg2.material_mut().ambient = 0.2;
    leg2.material_mut().diffuse = 0.7;
    world.add_object(leg2);

    /* Leg 3 */
    let mut leg3 = SimpleObject::cube();
    *leg3.transform_mut() = Matrix4::translation(-2.7, 1.5, -1.7) * Matrix4::scaling(0.1, 1.5, 0.1);
    leg3.material_mut().color = Color::new(0.5529, 0.4235, 0.3255);
    leg3.material_mut().ambient = 0.2;
    leg3.material_mut().diffuse = 0.7;
    world.add_object(leg3);

    /* Leg 4 */
    let mut leg4 = SimpleObject::cube();
    *leg4.transform_mut() = Matrix4::translation(-2.7, 1.5, 1.7) * Matrix4::scaling(0.1, 1.5, 0.1);
    leg4.material_mut().color = Color::new(0.5529, 0.4235, 0.3255);
    leg4.material_mut().ambient = 0.2;
    leg4.material_mut().diffuse = 0.7;
    world.add_object(leg4);

    /* ----------------------------- */

    /* Glass cube */
    let mut glass_cube = SimpleObject::cube();
    *glass_cube.transform_mut() = Matrix4::translation(0., 3.45001, 0.)
        * Matrix4::rotation_y(0.2)
        * Matrix4::scaling(0.25, 0.25, 0.25);
    // TODO: It looks like we implement this in chapter 16.
    // glass_cube.drop_shadow = false;
    glass_cube.material_mut().color = Color::new(1., 1., 0.8);
    glass_cube.material_mut().ambient = 0.;
    glass_cube.material_mut().diffuse = 0.3;
    glass_cube.material_mut().specular = 0.9;
    glass_cube.material_mut().shininess = 300.;
    glass_cube.material_mut().reflective = 0.7;
    glass_cube.material_mut().transparency = 0.7;
    glass_cube.material_mut().refractive_index = 1.5;
    world.add_object(glass_cube);

    /* Little cube 1 */
    let mut lil_cube1 = SimpleObject::cube();
    *lil_cube1.transform_mut() = Matrix4::translation(1., 3.35, -0.9)
        * Matrix4::rotation_y(-0.4)
        * Matrix4::scaling(0.15, 0.15, 0.15);
    lil_cube1.material_mut().color = Color::new(1., 0.5, 0.5);
    lil_cube1.material_mut().reflective = 0.6;
    lil_cube1.material_mut().diffuse = 0.4;
    world.add_object(lil_cube1);

    /* Little cube 2 */
    let mut lil_cube2 = SimpleObject::cube();
    *lil_cube2.transform_mut() = Matrix4::translation(-1.5, 3.27, 0.3)
        * Matrix4::rotation_y(0.4)
        * Matrix4::scaling(0.15, 0.07, 0.15);
    lil_cube2.material_mut().color = Color::new(1., 1., 0.5);
    world.add_object(lil_cube2);

    /* Little cube 3 */
    let mut lil_cube3 = SimpleObject::cube();
    *lil_cube3.transform_mut() = Matrix4::translation(0., 3.25, 1.)
        * Matrix4::rotation_y(0.4)
        * Matrix4::scaling(0.2, 0.05, 0.05);
    lil_cube3.material_mut().color = Color::new(0.5, 1., 0.5);
    world.add_object(lil_cube3);

    /* Little cube 4 */
    let mut lil_cube4 = SimpleObject::cube();
    *lil_cube4.transform_mut() = Matrix4::translation(-0.6, 3.4, -1.)
        * Matrix4::rotation_y(0.8)
        * Matrix4::scaling(0.05, 0.2, 0.05);
    lil_cube4.material_mut().color = Color::new(0.5, 0.5, 1.);
    world.add_object(lil_cube4);

    /* Little cube 5 */
    let mut lil_cube5 = SimpleObject::cube();
    *lil_cube5.transform_mut() = Matrix4::translation(2., 3.4, 1.)
        * Matrix4::rotation_y(0.8)
        * Matrix4::scaling(0.05, 0.2, 0.05);
    lil_cube5.material_mut().color = Color::new(0.5, 1., 1.);
    world.add_object(lil_cube5);

    /* ----------------------------- */

    /* Frame 1 */
    let mut frame1 = SimpleObject::cube();
    *frame1.transform_mut() = Matrix4::translation(-10., 4., 1.) * Matrix4::scaling(0.05, 1., 1.);
    frame1.material_mut().color = Color::new(0.7098, 0.2471, 0.2196);
    frame1.material_mut().diffuse = 0.6;
    world.add_object(frame1);

    /* Frame 2 */
    let mut frame2 = SimpleObject::cube();
    *frame2.transform_mut() =
        Matrix4::translation(-10., 3.4, 2.7) * Matrix4::scaling(0.05, 0.4, 0.4);
    frame2.material_mut().color = Color::new(0.2667, 0.2706, 0.6902);
    frame2.material_mut().diffuse = 0.6;
    world.add_object(frame2);

    /* Frame 3 */
    let mut frame3 = SimpleObject::cube();
    *frame3.transform_mut() =
        Matrix4::translation(-10., 4.6, 2.7) * Matrix4::scaling(0.05, 0.4, 0.4);
    frame3.material_mut().color = Color::new(0.3098, 0.5961, 0.3098);
    frame3.material_mut().diffuse = 0.6;
    world.add_object(frame3);

    /* ----------------------------- */

    /* Mirror */
    let mut mirror = SimpleObject::cube();
    *mirror.transform_mut() =
        Matrix4::translation(-2., 3.5, 9.95) * Matrix4::scaling(4.8, 1.4, 0.06);
    mirror.material_mut().color = Color::new(0., 0., 0.);
    mirror.material_mut().diffuse = 0.;
    mirror.material_mut().ambient = 0.;
    mirror.material_mut().specular = 1.;
    mirror.material_mut().shininess = 300.;
    mirror.material_mut().reflective = 1.;
    world.add_object(mirror);

    let mut camera = Camera::new(width as i32, height as i32, 0.785);
    camera.transform = transformations::view_transform(
        Tuple::point(8., 6., -8.),
        Tuple::point(0., 3., 0.),
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
