mod camera;
mod canvas;
mod color;
mod intersection;
mod light;
mod material;
mod matrix2;
mod matrix3;
mod matrix4;
mod misc;
mod pattern;
mod plane;
mod ray;
mod shape;
mod sphere;
mod transformations;
mod tuple;
mod world;
use canvas::Canvas;
use color::Color;
use glium::{
    backend::Facade, glutin, implement_vertex, texture::SrgbTexture2d, uniform,
    uniforms::UniformsStorage, Frame, Surface,
};
use light::Light;
use matrix4::Matrix4;
use std::f64::consts::PI;
use tuple::Tuple;

use crate::{camera::Camera, material::Material, pattern::Pattern, shape::Object, world::World};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn draw_world(camera_position: Tuple, looking_to: Tuple, light_position: Tuple) -> Canvas {
    let mut floor = Object::plane();
    *floor.material_mut() = Material::with_pattern(Pattern::striped(Color::red(), Color::green()));
    floor.material_mut().color = Color::new(1., 0.9, 0.9);
    floor.material_mut().specular = 0.;

    let mut middle = Object::sphere();
    *middle.transform_mut() = Matrix4::translation(-0.5, 1., 0.5);
    *middle.material_mut() = Material::with_pattern(Pattern::striped(Color::red(), Color::green()));
    middle.material_mut().color = Color::new(0.1, 1., 0.5);
    middle.material_mut().diffuse = 0.7;
    middle.material_mut().specular = 0.3;

    let mut right = Object::sphere();
    *right.transform_mut() = Matrix4::translation(1.5, 0.5, -0.5) * Matrix4::scaling(0.5, 0.5, 0.5);
    *right.material_mut() = Material::with_pattern(Pattern::striped(Color::red(), Color::green()));
    right.material_mut().color = Color::new(0.5, 1., 0.1);
    right.material_mut().diffuse = 0.7;
    right.material_mut().specular = 0.3;

    let mut left = Object::sphere();
    *left.transform_mut() =
        Matrix4::translation(-1.5, 0.33, -0.75) * Matrix4::scaling(0.33, 0.33, 0.33);
    *left.material_mut() = Material::with_pattern(Pattern::striped(Color::red(), Color::green()));
    left.material_mut().color = Color::new(1., 0.8, 0.1);
    left.material_mut().diffuse = 0.7;
    left.material_mut().specular = 0.3;

    let mut world = World::new();
    world.objects = vec![floor, middle, right, left];
    world.light = Some(Light::point_light(light_position, Color::white()));

    let mut camera = Camera::new(WIDTH as i32, HEIGHT as i32, PI / 3.);
    camera.transform =
        transformations::view_transform(camera_position, looking_to, Tuple::vector(0., 1., 0.));

    camera.render(world)
}

fn degrees(deg: f64) -> f64 {
    deg * PI / 180.0
}

fn rotate_around_point(to_rotate: Tuple, point: Tuple) -> Tuple {
    Matrix4::translation(point.x, point.y, point.z)
        * Matrix4::rotation_y(degrees(1.0))
        * Matrix4::translation(-point.x, -point.y, -point.z)
        * to_rotate
}
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coord);

fn main() {
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex1 = Vertex {
        position: [-1.0, 1.0],
        tex_coord: [0.0, 0.95],
    };
    let vertex2 = Vertex {
        position: [-1.0, -1.0],
        tex_coord: [0.0, 0.0],
    };
    let vertex3 = Vertex {
        position: [1.0, 1.0],
        tex_coord: [1.0, 0.95],
    };
    let vertex4 = Vertex {
        position: [1.0, -1.0],
        tex_coord: [1.0, 0.0],
    };
    let mut top_triangle = vec![vertex1, vertex2, vertex3];
    let mut bottom_triangle = vec![vertex2, vertex4, vertex3];
    let mut shapes = vec![];
    shapes.append(&mut top_triangle);
    shapes.append(&mut bottom_triangle);

    let vertex_shader_src = r#"
    #version 140

    in vec2 position;
    in vec2 tex_coord;
    out vec2 v_tex;
    uniform sampler2D tex;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
        v_tex = tex_coord;
    }
"#;
    let fragment_shader_src = r#"
    #version 140

    in vec2 v_tex;
    out vec4 color;
    uniform sampler2D tex;

    void main() {
        color = texture(tex, v_tex);
    }
"#;

    let vertex_buffer = glium::VertexBuffer::new(&display, &shapes).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut camera_position = Matrix4::scaling(3., 6., 3.) * Tuple::point(0., 1.5, -5.);
    let looking_to = Tuple::point(0., 1., 0.);
    let mut light_position = Tuple::point(-10., 10., -10.);

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let canvas = draw_world(camera_position, looking_to, light_position);
        // camera_position = rotate_around_point(camera_position, looking_to);
        light_position.y += 0.1;
        let pixel_data = canvas
            .pixels
            .into_iter()
            .flat_map(|c| vec![c.red as f32, c.green as f32, c.blue as f32])
            .collect::<Vec<_>>();
        let image = glium::texture::RawImage2d::from_raw_rgb_reversed(
            &pixel_data,
            (canvas.width as u32, canvas.height as u32),
        );
        let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

        let uniforms = uniform! {
             tex: &texture
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
