use image::{RgbImage, ImageBuffer, Rgb};

mod vec3;
mod ray;
mod surface;
mod constants;
mod scene;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::{Surface, hit_record, Sphere};
use crate::scene::{Scene};

fn main() {
	// image setup
	let aspect_ratio: f64 = 16.0 / 9.0;
	let image_width: u32 = 256;
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

	let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
	let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
	let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
	let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x: 0.0, y: 0.0, z: focal_length};

	// render

	let mut buffer: RgbImage = ImageBuffer::new(image_width, image_height);

	for (x, y, pixel) in buffer.enumerate_pixels_mut() {
		// big y = bottom of image
		// so we invert it using this cursed code
		let y: u32 = (-(y as i32) + image_height as i32) as u32;
		let u: f64 = x as f64 / (image_width-1) as f64;
		let v: f64 = y as f64 / (image_height-1) as f64;
		
		let ray: Ray = Ray { origin: origin, direction: lower_left_corner + horizontal*u + vertical*v - origin };

		let pixel_color: Vec3 = ray_color(ray, &scene);

		// convert to 255 range
		let ir = (255.999 * pixel_color.x) as u8;
		let ig = (255.999 * pixel_color.y) as u8;
		let ib = (255.999 * pixel_color.z) as u8;

		*pixel = Rgb([ir, ig, ib]);
	}

	match buffer.save("image.png") {
		Err(e) => eprintln!("Error writing file: {}", e),
		Ok(()) => println!("Done."),
};
}

fn ray_color(r: Ray, world: &Scene) -> Vec3 {
	let rec = world.hit(r, 0.0, constants::INFINITY);
	if(rec.hit_anything) {
		return (rec.normal + Vec3{x: 1.0, y: 1.0, z: 1.0}) * 0.5;
	}
	let unit_direction = r.direction.unit_vector();
	let t = 0.5*(unit_direction.y + 1.0);
	return Vec3 {x: 1.0, y: 1.0, z: 1.0}*(1.0-t) + Vec3{x: 0.5, y: 0.7, z: 1.0}*t;
}