use image::{RgbImage, ImageBuffer, Rgb};

mod vec3;
mod ray;
mod surface;
mod utility;
mod scene;
mod camera;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::{Sphere, Lambert, Metal, Dielectric};
use crate::scene::{Scene};
use crate::camera::{Camera};
use rand::Rng;

fn main() {
	println!("Initializing...");
	let mut rng = rand::thread_rng();
	
	// image setup
	let aspect_ratio: f64 = 16.0 / 9.0;
	let image_width: u32 = 100;
	let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
	let num_samples = 10;
	let max_depth = 10;

	// world setup
	let scene = Scene::random_scene();

	// camera
	let look_from = Vec3::new(13.0,2.0,3.0);
	let look_at = Vec3::new(0.0,0.0,0.0);
	let v_up = Vec3::new(0.0,1.0,0.0);
	let fov = 20.0;
	let aperture = 0.1;
	let focus_dist = 10.0;
	
	let camera: Camera = Camera::new(
		look_from,
		look_at,
		v_up,
		fov,
		aspect_ratio,
		aperture,
		focus_dist,
	);

	// render
	let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);

	println!("Beginning Render...");

	let mut row: u32 = 0;
	
	for (x, y, pixel) in buffer.enumerate_pixels_mut() {
		// track progress
		if y != row {
			row = y;
			println!("Row: {} of {} ({}%)", row, image_height, ((row as f64 / image_height as f64) * 100.0).round());
		}
		
		// big y = bottom of image
		// so we invert it using this cursed code
		let y: u32 = (-(y as i32) + image_height as i32) as u32;

		let mut pixel_color = Vec3{x:0.0, y:0.0, z:0.0};
		
		for _s in 0..num_samples {
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
	let rec = world.hit(r, 0.001, utility::INFINITY);
	if rec.hit_anything {
		let scatter = rec.material.unwrap().scatter(r, rec);
		if scatter.0 {
			let attenuation = scatter.1;
			let scattered_ray = scatter.2;
			
			return ray_color(
				scattered_ray,
				world,
				depth - 1
				) * attenuation;
		}
		else{
			return Vec3{x:0.0,y:0.0,z:0.0};
		}
	}
	// background
	else{
		let unit_direction = r.direction.unit_vector();
		let t = 0.5*(unit_direction.y + 1.0);
		return Vec3 {x: 1.0, y: 1.0, z: 1.0}*(1.0-t) + Vec3{x: 0.5, y: 0.7, z: 1.0}*t;
	}
	
}