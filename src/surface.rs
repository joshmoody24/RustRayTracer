use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::utility::{random_unit_vector, random_in_unit_sphere, random_double};

pub struct HitRecord<'a> {
	pub hit_anything: bool,
	pub p: Vec3,
	pub normal: Vec3,
	pub t: f64,
	pub front_face: bool,
	pub material: Option<&'a Box<dyn Scatter>>
}

impl HitRecord<'_> {
	pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3){
		self.front_face = r.direction.dot(outward_normal) < 0.0;
		self.normal = if self.front_face {outward_normal} else {-outward_normal};
	}
	pub const NO_HIT: HitRecord<'static> = HitRecord {
		hit_anything: false,
		p: Vec3::ZERO,
		normal: Vec3::ZERO,
		t: 0.0,
		front_face: true,
		material: None
	};
}

pub trait Hit {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRecord;
}

pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
	pub material: Box<dyn Scatter>,
}

impl Hit for Sphere {
	fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRecord {
		let oc: Vec3 = r.origin - self.center;
		let a: f64 = r.direction.length_squared();
		let half_b = oc.dot(r.direction);
		let c = oc.length_squared() - self.radius*self.radius;

		let discriminant: f64 = half_b*half_b - a*c;
		if discriminant < 0.0 {
			return HitRecord::NO_HIT;
		}
		let sqrtd = discriminant.sqrt();

		// find the nearest root that lies in the acceptable range
		let mut root = (-half_b - sqrtd) / a;
		if root < t_min || t_max < root {
			root = (-half_b + sqrtd) / a;
			if root < t_min || t_max < root {
				return HitRecord::NO_HIT;
			}
		}
			
		let mut rec: HitRecord = HitRecord {
			t: root,
			p: r.at(root),
			normal: (r.at(root) - self.center) / self.radius,
			hit_anything: true,
			front_face: true,
			material: Some(&self.material)
		};
		let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
		rec.set_face_normal(r, outward_normal);
		return rec;
	}
}


// materials 

// the method used for all materials
// that defines how they scatter light
pub trait Scatter {
	fn scatter(&self, r_in: Ray, rec: HitRecord) -> (bool, Vec3, Ray);
}

pub struct Lambert {
	pub albedo: Vec3,
}

impl Scatter for Lambert{
	fn scatter(&self, _r_in: Ray, rec: HitRecord) -> (bool, Vec3, Ray) {
		let mut scatter_direction = rec.normal + random_unit_vector();
		if scatter_direction.near_zero() {
			scatter_direction = rec.normal;
		}
		let output_ray = Ray{origin: rec.p, direction: scatter_direction};
		let output_attenuation = self.albedo;
		return (true, output_attenuation, output_ray);
	}
}

pub struct Metal {
	pub albedo: Vec3,
	pub roughness: f64,
}

impl Scatter for Metal {
	fn scatter(&self, r_in: Ray, rec: HitRecord) -> (bool, Vec3, Ray) {
		let reflected: Vec3 = reflect(r_in.direction.unit_vector(), rec.normal);
		let scattered = Ray{origin:rec.p, direction:reflected+random_in_unit_sphere()*self.roughness};
		let attenuation = self.albedo;
		let can_scatter = scattered.direction.dot(rec.normal) > 0.0;
		return (can_scatter, attenuation, scattered);
	}
}

pub struct Dielectric {
	pub ior: f64,
}

impl Scatter for Dielectric {
	fn scatter(&self, r_in: Ray, rec: HitRecord) -> (bool, Vec3, Ray) {
		let attenuation = Vec3{x:1.0,y:1.0,z:1.0};
		let refraction_ratio = if rec.front_face {1.0/self.ior} else {self.ior};
		
		let unit_direction = r_in.direction.unit_vector();
		let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
		let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
		
		let cannot_refract = refraction_ratio * sin_theta > 1.0;
		let direction: Vec3;

		if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() {
			direction = reflect(unit_direction, rec.normal);
		}
		else{
			direction = refract(unit_direction, rec.normal, refraction_ratio);
		}

		let scattered = Ray{origin:rec.p, direction:direction};
		return(true, attenuation, scattered);
	}
}

impl Dielectric {
	fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
		// Use Schlick's approximation for reflectance
		let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
		r0 = r0*r0;
		return r0 + (1.0-r0)*(1.0-cosine).powf(5.0);
	}
}

// reflects a ray like a mirror
fn reflect(v: Vec3, n: Vec3) -> Vec3 {
	return v - n*v.dot(n)*2.0;
}

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
	let cos_theta = n.dot(-uv).min(1.0);
	let r_out_perp = (uv + n*cos_theta) * etai_over_etat;
	let r_out_parallel = n * -(((1.0 - r_out_perp.length_squared()) as f64).abs().sqrt());
	return r_out_perp + r_out_parallel;
}