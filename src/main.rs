mod canvas;
mod color;
mod misc;
mod tuple;
use canvas::Canvas;
use color::Color;
use tuple::Tuple;
mod matrix;

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

fn perform_simulation(canvas: &mut Canvas) {
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
        let y = canvas.height as i32 - projectile.position.y.round() as i32;

        canvas.write_pixel(x, y, Color::red());
        projectile = tick(env, projectile);
    }
}

fn main() {
    let mut canvas = Canvas::new(900, 550);
    perform_simulation(&mut canvas);

    println!("{}", canvas.to_ppm());
}
