mod tuple;
use tuple::Tuple;

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

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0., 1., 0.),
        velocity: Tuple::vector(1., 1., 0.).normalize() * 1500.,
    };
    let e = Environment {
        gravity: Tuple::vector(0., -0.1, 0.),
        wind: Tuple::vector(-0.01, 0., 0.),
    };

    let mut ticks = 0;
    while p.position.y > 0.0 {
        p = tick(e, p);
        ticks += 1;
        println!("{:?}", p.position);
    }
    println!("Ticks taken = {}", ticks);
}
