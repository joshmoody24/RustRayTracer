use crate::ray::Ray;
use crate::surface::{hit_record, Surface, Hit};
use std::boxed::Box;

pub struct Scene {
	pub surfaces: Vec<Box<dyn Hit>>,
}

impl Scene {
	pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> hit_record {
		let mut rec: hit_record = hit_record::NO_HIT;
		let mut closest_so_far = t_max;

		for s in &self.surfaces {
			let temp_rec = s.hit(r, t_min, closest_so_far);
			if temp_rec.hit_anything {
				// TODO: is this correct?
				closest_so_far = temp_rec.t;
				rec = temp_rec;
			}
		}

		return rec;
	}
}