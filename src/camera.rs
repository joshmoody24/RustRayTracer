use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::surface::{Surface, hit_record, Sphere};
use crate::scene::{Scene};

pub struct Camera {
	pub origin: Vec3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub lower_left_corner: Vec3,
}

impl Camera {
	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray {
			origin: self.origin,
			direction: self.lower_left_corner + self.horizontal*u + self.vertical*v - self.origin
		}
	}
}