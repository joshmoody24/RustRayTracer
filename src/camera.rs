use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::utility::{degrees_to_radians, random_between};

pub struct Camera {
	pub origin: Vec3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub lower_left_corner: Vec3,
	u: Vec3,
	v: Vec3,
	w: Vec3,
	pub lens_radius: f64	
}

impl Camera {
	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		let rd: Vec3 = random_in_unit_disk() * self.lens_radius;
		let offset: Vec3 = self.u*rd.x + self.v*rd.y;
		Ray {
			origin: self.origin + offset,
			direction: self.lower_left_corner + self.horizontal*s + self.vertical*t - self.origin - offset
		}
	}

	// vfov = vertical field of view in degrees
	pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Camera {
		let theta = degrees_to_radians(vfov);
		let h = (theta/2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = (look_from - look_at).unit_vector();
		let u = v_up.cross(w).unit_vector();
		let v = w.cross(u);
		
		let origin = look_from;
		let horizontal = u*viewport_width*focus_dist;
		let vertical = v*viewport_height*focus_dist;
		let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w*focus_dist;

		let lens_radius = aperture / 2.0;

		return Camera {
			origin,
			horizontal,
			vertical,
			lower_left_corner,
			u,
			v,
			w,
			lens_radius
		}
	}
}

fn random_in_unit_disk() -> Vec3 {
	loop {
		let p = Vec3{x:random_between(-1.0,1.0), y:random_between(-1.0,1.0), z:0.0};
		if p.length_squared() >= 1.0 {continue};
		return p;
	}
}