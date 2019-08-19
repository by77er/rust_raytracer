use std::fs::File;
use std::io::prelude::*;

extern crate rand;
use rand::Rng;

#[macro_use]
mod gfx;
use gfx::*;

#[macro_use]
mod ppm;
use ppm::*;

const FILENAME: &'static str =  "out.ppm";
const DIMS: (u16, u16) = (500, 250);
const AA_ROUNDS: u16 = 100;

fn main() {
    let mut world = HitableList {
        list: Vec::new()
    };
    world.list.push(Shapes::Sphere(Sphere {
        center: vec3!(0.0, 0.0, -1.0),
        radius: 0.5
    }));
    world.list.push(Shapes::Sphere(Sphere {
        center: vec3!(0.0, -100.5, -1.0),
        radius: 100.0
    }));

    let cam = Camera::new();
    let mut rng = rand::thread_rng();

    let mut img = PPM::new(DIMS.0, DIMS.1);
    for y in 0..DIMS.1 {
        for x in 0..DIMS.0 {
            let mut color = vec3!(0.0, 0.0, 0.0);
            for _ in 0..AA_ROUNDS {
                let u = (x as f32 + rng.gen::<f32>()) / DIMS.0 as f32;
                let v = (y as f32 + rng.gen::<f32>()) / DIMS.1 as f32;
                let ray = cam.get_ray(u, v);
                // let p = ray.point_at_parameter(2.0);
                color += ray_color(&ray, Shapes::HitableList(world.clone()));
            }
            color /= AA_ROUNDS as f32;

            img.set(x, DIMS.1 - y - 1, pxl!(
                (255.99 * color[0].sqrt()) as u8,
                (255.99 * color[1].sqrt()) as u8,
                (255.99 * color[2].sqrt()) as u8
            )).unwrap();
        }
    }
    write_file(img.to_string()).unwrap();
}

fn write_file(strn: String) -> std::io::Result<()> {
    println!("Wrote {}x{} to {}", DIMS.0, DIMS.1, FILENAME);
    let mut output = File::create(FILENAME)?;
    output.write_all(strn.to_string().as_bytes())?;
    output.sync_all()?;
    Ok(())
}