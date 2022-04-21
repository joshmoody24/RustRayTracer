use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utility::{degrees_to_radians};

pub struct Camera {
	pub origin: Vec3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub lower_left_corner: Vec3,
}

impl Camera {
	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		Ray {
			origin: self.origin,
			direction: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin
		}
	}

	// vfov = vertical field of view in degrees
	pub fn create(look_from: Vec3, look_at: Vec3, v_up: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
		let theta = degrees_to_radians(vfov);
		let h = (theta/2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = (look_from - look_at).unit_vector();
		let u = v_up.cross(w).unit_vector();
		let v = w.cross(u);
		
		let origin = look_from;
		let horizontal = u*viewport_width;
		let vertical = v*viewport_height;
		let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

		return Camera {
			origin,
			horizontal,
			vertical,
			lower_left_corner 
		}
	}
}