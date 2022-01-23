use std::{fs::File, io::Write};

use ray_tracer::{camera::Camera, world::World};

pub fn run_and_save_scene(example_name: &str, camera: Camera, world: World) {
    let file_name = format!("./{}.ppm", example_name);
    println!("Writing scene to: {}", file_name);

    let ppm = camera.render(&world).to_ppm();

    let mut f = File::create(&file_name).expect("Unable to create file");
    f.write_all(ppm.as_bytes()).expect("Unable to write data");
}
