use image::{RgbImage, ImageBuffer, Rgb};

mod vec3;
mod ray;
mod surface;
mod constants;
mod scene;
mod camera;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::{Surface, hit_record, Sphere};
use crate::scene::{Scene};
use crate::camera::{Camera};
use rand::Rng;
use crate::constants::{random_unit_vector};

fn main() {

	let mut rng = rand::thread_rng();
	
	// image setup
	let aspect_ratio: f64 = 16.0 / 8.0;
	let image_width: u32 = 1024;
	let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

	// world setup
	let mut scene: Scene = Scene {
		surfaces: Vec::new()
	};
	scene.surfaces.push(Box::new(Sphere {
		center: Vec3 {
			x: 0.0,
			y: 0.0,
			z: -1.0,
		},
		radius: 0.5
	}));

	// floor
	scene.surfaces.push(Box::new(Sphere {
		center: Vec3 {
			x: 0.0,
			y: -100.5,
			z: -1.0,
		},
		radius: 100.0
	}));

	// camera setup
	let viewport_height: f64 = 2.0;
	let viewport_width: f64 = aspect_ratio * viewport_height;
	let focal_length: f64 = 1.0;

	// anti aliasing
	let num_samples = 200;
	let max_depth = 8;

	let cam_origin = Vec3{x:0.0, y:0.0, z:0.0};
	let cam_horizontal = Vec3{x:viewport_width, y:0.0, z:0.0};
	let cam_vertical = Vec3{x:0.0, y: viewport_height, z:0.0};
	let cam_lower_left_corner = cam_origin - cam_horizontal / 2.0 - cam_vertical / 2.0 - Vec3 {x:0.0, y: 0.0, z: focal_length };

	let camera: Camera = Camera {
		lower_left_corner: cam_lower_left_corner,
		horizontal: cam_horizontal,
		vertical: cam_vertical,
		origin: cam_origin,
	};

	// render

	let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);

	for (x, y, pixel) in buffer.enumerate_pixels_mut() {
		// big y = bottom of image
		// so we invert it using this cursed code
		let y: u32 = (-(y as i32) + image_height as i32) as u32;

		let mut pixel_color = Vec3{x:0.0, y:0.0, z:0.0};
		
		for s in 0..num_samples {
			let u: f64 = (x as f64 + rng.gen::<f64>()) / (image_width-1) as f64;
			let v: f64 = (y as f64 + rng.gen::<f64>()) / (image_height-1) as f64;
			let ray: Ray = camera.get_ray(u, v);
			pixel_color = pixel_color + ray_color(ray, &scene, max_depth);
		}

		// divide color by num samples and gamma correrct for gamma=2.0
		let scale = 1.0 / num_samples as f64;
		let r = (scale*pixel_color.x).sqrt();
		let g = (scale*pixel_color.y).sqrt();
		let b = (scale*pixel_color.z).sqrt();
		
		// convert to 255 range
		let ir = (255.999 * r) as u8;
		let ig = (255.999 * g) as u8;
		let ib = (255.999 * b) as u8;

		*pixel = Rgb([ir, ig, ib]);
	}

	match buffer.save("image.png") {
		Err(e) => eprintln!("Error writing file: {}", e),
		Ok(()) => println!("Done."),
};
}

fn ray_color(r: Ray, world: &Scene, depth: u32) -> Vec3 {
	if depth <= 0 {
		return Vec3 {x: 0.0, y: 0.0, z: 0.0};
	}
	// second param is the bias
	let rec = world.hit(r, 0.001, constants::INFINITY);
	if rec.hit_anything {
		let target: Vec3 = rec.p + rec.normal + random_unit_vector();
		return ray_color(
			Ray{origin: rec.p, direction: target-rec.p},
			world,
			depth - 1
			) * 0.5;
	}
	// background
	else{
		let unit_direction = r.direction.unit_vector();
		let t = 0.5*(unit_direction.y + 1.0);
		return Vec3 {x: 1.0, y: 1.0, z: 1.0}*(1.0-t) + Vec3{x: 0.5, y: 0.7, z: 1.0}*t;
	}
	
}