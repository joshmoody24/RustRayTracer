use crate::ray::Ray;
use crate::surface::{HitRecord, Hit};
use std::boxed::Box;

pub struct Scene {
	pub surfaces: Vec<Box<dyn Hit>>,
}

impl Scene {
	pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRecord {
		let mut rec: HitRecord = HitRecord::NO_HIT;
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
/*
	pub fn random_scene() -> &Vec<Box<dyn Hit>> {
		let scene = Vec::new();
			// materials setup
		//let floor_material = Box::new(Lambert {albedo: Vec3{x: 0.8, y: 0.8, z: 0.0}});
		let diffuse_material = Box::new(Lambert {albedo: Vec3{x: 0.1, y: 0.2, z: 0.5}});
		//let metal_material = Box::new(Metal {albedo: Vec3{x: 0.8, y: 0.8, z: 0.8}, roughness: 0.1});
		//let glass_material = Box::new(Dielectric {ior: 1.33});

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
			radius: 0.5,
			material: diffuse_material
		}));
		
		return &scene;
	}
*/
}