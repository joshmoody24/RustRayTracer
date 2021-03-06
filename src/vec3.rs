use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::utility::{random_double, random_between};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Add for Vec3 {
	type Output = Vec3;
	fn add(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
	}
}

impl Sub for Vec3 {
	type Output = Vec3;
	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;
	fn mul(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
	}
}

impl Mul<f64> for Vec3 {
	type Output = Vec3;
	fn mul(self, other: f64) -> Vec3 {
		Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
	}
}


impl Div<Vec3> for Vec3 {
	type Output = Vec3;
	fn div(self, other: Vec3) -> Vec3 {
		Vec3 { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
	}
}

impl Div<f64> for Vec3 {
	type Output = Vec3;
	fn div(self, other: f64) -> Vec3 {
		Vec3 { x: self.x / other, y: self.y / other, z: self.z / other }
	}
}

impl Neg for Vec3 {
	type Output = Vec3;
	fn neg(self) -> Vec3 {
		Vec3 { x: -self.x, y: -self.y, z: -self.z }
	}
}

impl Vec3 {
	pub fn new(x: f64, y:f64, z:f64) -> Vec3{
		return Vec3{x,y,z};	
	}
	pub fn length(self) -> f64 {
		let ls = self.length_squared();
		ls.sqrt()
	}
	pub fn length_squared(self) -> f64 {
		self.x * self.x + self.y * self.y + self.z * self.z
	}
	pub fn unit_vector(self) -> Vec3 {
		self / self.length()
	}
	pub fn dot(self, other: Vec3) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
	pub fn cross(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x
		}
	}
	pub fn near_zero(self) -> bool {
		let small: f64 = 1e-8;
		return self.x.abs() < small && self.y.abs() < small && self.z.abs() < small;
	}
	pub fn random() -> Vec3{
		return Vec3::new(random_double(), random_double(), random_double());
	}
	pub fn random_between(min: f64, max: f64) -> Vec3{
		return Vec3::new(random_between(min,max), random_between(min,max), random_between(min,max));
	}
	pub const ZERO: Vec3 = Vec3{x:0.0,y:0.0,z:0.0};
}