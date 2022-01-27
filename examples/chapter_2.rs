use std::fs::File;
use std::io::Write;

use examples::output_file_path;
use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::math::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Clone, Copy)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn perform_simulation(width: usize, height: usize) -> Canvas {
    let mut canvas = Canvas::new(width, height);

    let mut projectile = Projectile {
        position: Tuple::point(0., 1., 0.),
        velocity: Tuple::vector(1., 1.8, 0.).normalize() * 11.25,
    };
    let env = Environment {
        gravity: Tuple::vector(0., -0.1, 0.),
        wind: Tuple::vector(-0.01, 0., 0.),
    };

    while projectile.position.y > 0.0 {
        let x = projectile.position.x.round() as i32;
        let y = canvas.height() as i32 - projectile.position.y.round() as i32;

        canvas.write_pixel(x, y, Color::red());
        projectile = tick(env, projectile);
    }

    canvas
}

const ASPECT: f64 = 16. / 9.;

const WIDTH: usize = 900;
const HEIGHT: usize = (WIDTH as f64 / ASPECT) as usize;

pub fn main() {
    let file_name = output_file_path("chapter_2");
    println!("Writing scene to: {}", file_name);

    let canvas = perform_simulation(WIDTH, HEIGHT);
    let ppm = canvas.to_ppm();

    let mut f = File::create(&file_name).expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
