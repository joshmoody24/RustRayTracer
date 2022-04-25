use crate::ray::Ray;
use crate::surface::{HitRecord, Hit, Sphere, Dielectric, Metal, Lambert, Scatter};
use std::boxed::Box;
use crate::vec3::Vec3;
use crate::utility::{random_double, random_between};

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

	pub fn random_scene() -> Scene {
		let scene = Vec::<Box<dyn Hit>>::new();

		// world setup
		let mut scene: Scene = Scene {
			surfaces: Vec::new()
		};
		
		
		let floor_material = Box::new(Lambert {albedo: Vec3{x: 0.5, y: 0.5, z: 0.5}});
		scene.surfaces.push(Box::new(Sphere {
			center: Vec3 {
				x: 0.0,
				y: -1_000.0,
				z: -1.0,
			},
			radius: 1_000.0,
			material: floor_material
		}));

		for a in -11..11 {
			for b in -11..11 {
				let choose_mat = random_double();
				let center = Vec3::new((a as f64)+0.9*random_double(), 0.2, (b as f64)+0.9*random_double());

				// keep the baby spheres away from the big center spheres
				if (center - Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
					let sphere_material: Box<dyn Scatter>;
					if choose_mat < 0.8 {
						// diffuse
						let albedo = Vec3::random() * Vec3::random();
						sphere_material = Box::new(Lambert{albedo});
					}
					else if choose_mat < 0.95 {
						// metal
						let albedo = Vec3::random_between(0.5, 1.0);
						let roughness = random_between(0.0, 0.5);
						sphere_material = Box::new(Metal{albedo, roughness});
					}
					else{
						sphere_material = Box::new(Dielectric{ior: 1.5});
					}
					
					scene.surfaces.push(Box::new(Sphere{center, radius:0.2, material: sphere_material}))
				}
			}
		}

		// add the big spheres
		let material1 = Box::new(Dielectric{ior:1.5});
		let material2 = Box::new(Lambert{albedo:Vec3::new(0.4,0.2,0.1)});
		let material3 = Box::new(Metal{albedo:Vec3::new(0.7,0.6,0.5), roughness: 0.0});
		scene.surfaces.push(Box::new(Sphere{center:Vec3::new(0.0,1.0,0.0), radius:1.0, material: material1}));
		scene.surfaces.push(Box::new(Sphere{center:Vec3::new(-4.0,1.0,0.0), radius:1.0, material: material2}));
		scene.surfaces.push(Box::new(Sphere{center:Vec3::new(4.0,1.0,0.0), radius:1.0, material: material3}));
		
		scene
	}
}