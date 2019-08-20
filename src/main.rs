extern crate image;
use image::{Rgb, ImageBuffer, RgbImage};

extern crate rand;
use rand::Rng;

mod raytracer;
use raytracer::math::vec3::*;
use raytracer::camera::*;
use raytracer::object::*;

const FILENAME: &'static str = "render.png"; // Output filename
const DIMS: (u32, u32) = (1000, 500);         // Image dimensions
const AA_ROUNDS: u16 = 100;                  // Samples per pixel

fn main() {
    let mut img: RgbImage = ImageBuffer::new(DIMS.0, DIMS.1);
    render(&mut img);
    match img.save(FILENAME) {
        Ok(_) => println!("Saved {}x{} output as {}", DIMS.0, DIMS.1, FILENAME),
        Err(e) => println!("Failed to save {}: {}", FILENAME, e)
    }
}

// Handles pretty much everything related to generating the image
fn render(img: &mut RgbImage) {
    // lets us create rays
    let cam = Camera::new();

    // Defines the world and its contents
    let sphere1 = Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5
    );
    let sphere2 = Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0
    );
    let mut world = World::new();
    world.add_object(Box::new(sphere1));
    world.add_object(Box::new(sphere2));

    // Random numbers for antialiasing
    let mut rng = rand::thread_rng();

    for x in 0..DIMS.0 {
        for y in 0..DIMS.1 {
            // Rendering logic goes here
            // Base color
            let mut color = Vec3::all(0.0);
            // Loops for antialiasing
            for _ in 0..AA_ROUNDS {
                let u = (x as f32 + rng.gen::<f32>()) / DIMS.0 as f32;
                let v = (y as f32 + rng.gen::<f32>()) / DIMS.1 as f32;
                let ray = cam.get_ray(u, v);
                let tmp_color = ray.get_color(&world); // Will have a ref to the world in the future
                color.add_by_vec_eq(&tmp_color);
            }
            // Average each sample
            color.div_eq(AA_ROUNDS as f32);
            // Write to pixel
            let out = color_transform(&color);
            img[(x, DIMS.1 - y - 1)] = Rgb([out.0, out.1, out.2]);
        }
    }
}

fn color_transform(triad: &Vec3) -> (u8, u8, u8) {
    (
        (triad.x.sqrt() * 255.99) as u8,
        (triad.y.sqrt() * 255.99) as u8,
        (triad.z.sqrt() * 255.99) as u8
    )
}