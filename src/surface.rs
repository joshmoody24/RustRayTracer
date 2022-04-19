use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct hit_record {
	pub hit_anything: bool,
	pub p: Vec3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
}

impl hit_record {
	pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3){
		self.front_face = r.direction.dot(outward_normal) < 0.0;
		self.normal = if self.front_face {outward_normal} else {-outward_normal};
	}
	pub const NO_HIT: hit_record = hit_record {
		hit_anything: false,
		p: Vec3::ZERO,
		normal: Vec3::ZERO,
		t: 0.0,
		front_face: true,
	};
}

pub trait Hit {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> hit_record;
}

pub struct Surface {
	object: dyn Hit
}

pub struct Sphere {
	pub center: Vec3,
	pub radius: f64
}

impl Hit for Sphere {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> hit_record {
		let oc: Vec3 = r.origin - self.center;
		let a: f64 = r.direction.length_squared();
		let half_b = oc.dot(r.direction);
		let c = oc.length_squared() - self.radius*self.radius;

		let discriminant: f64 = half_b*half_b - a*c;
		if discriminant < 0.0 {
			return hit_record::NO_HIT;
		}
		let sqrtd = discriminant.sqrt();

		// find the nearest root that lies in the acceptable range
		let mut root = (-half_b - sqrtd) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrtd) / a;
			if root < t_min || t_max < root {
				return hit_record::NO_HIT;
			}
		}

		let mut rec: hit_record = hit_record {
			t: root,
			p: r.at(root),
			normal: (r.at(root) - self.center) / self.radius,
			hit_anything: true,
			front_face: true,
		};
		let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
		rec.set_face_normal(r, outward_normal);

		return rec;
	}
}