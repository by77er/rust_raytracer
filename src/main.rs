extern crate image;
use image::{Rgb, ImageBuffer, RgbImage};

extern crate rand;
use rand::Rng;

mod raytracer;
use raytracer::math::vec3::*;
use raytracer::camera::*;
use raytracer::object::*;

const FILENAME: &'static str = "render.png"; // Output filename
const DIMS: (u32, u32) = (2000, 1000);         // Image dimensions
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
    let from = Vec3::new(13.0, 2.0, 3.0);
    let at   = Vec3::new(0.0, 0.0, 0.0);
    let cam = Camera::new(
            from,
            at,
            Vec3::new(0.0, 1.0, 0.0),
            20.0, // FOV
            DIMS.0 as f32 / DIMS.1 as f32, // Aspect ratio
            0.1, // Aperture
            10.0 // Focal length
        );

    // Defining the materials used in the scene
    let world = World::random();
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
                let tmp_color = ray.get_color(&world, 0);
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