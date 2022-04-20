pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

use crate::vec3::Vec3;
use rand::Rng;

// utility functions
pub fn degrees_to_radians(degrees: f64){
	degrees * PI / 180.0;
}

pub fn random_in_unit_sphere() -> Vec3 {
	let mut rng = rand::thread_rng();
	let mut p: Vec3;
	loop {
		p = (Vec3{x: rng.gen::<f64>(), y: rng.gen::<f64>(), z: rng.gen::<f64>()} * 2.0) - Vec3{x:1.0,y:1.0,z:1.0};
		if(p.length_squared() >= 1.0){
			break;
		}
	}
	return p;
}

pub fn random_unit_vector() -> Vec3{
	return random_in_unit_sphere().unit_vector();
}