use std::ops::{Add,Sub,Neg,Mul};

#[derive(Debug,Copy,Clone,Default)]
pub struct Vec4 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub w: f64,
}
// ------------------------------
impl Add for Vec4 {
	type Output = Vec4;
	fn add(self, o: Vec4) -> Vec4 {
		Vec4 { x: self.x + o.x, y: self.y + o.y, z: self.z + o.z, w: 1. }
	}
}
// ------------------------------
impl Sub for Vec4 {
	type Output = Vec4;
	fn sub(self, o: Vec4) -> Vec4 {
		Vec4 { x: self.x - o.x, y: self.y - o.y, z: self.z - o.z, w: 1. }
	}
}
// ------------------------------
impl Neg for Vec4 {
	type Output = Vec4;
	fn neg(self) -> Vec4 {
		Vec4 { x: -self.x, y: -self.y, z: -self.z, w: 1. }
	}
}
// ------------------------------
impl Mul<f64> for Vec4 {
	type Output = Vec4;
	fn mul(self, o: f64) -> Vec4 {
		Vec4 { x: self.x * o, y: self.y * o, z: self.z * o, w: 1. }
	}
}
impl Mul<Vec4> for f64 {
	type Output = Vec4;
	fn mul(self, o: Vec4) -> Vec4 {
		Vec4 { x: self * o.x, y: self * o.y, z: self * o.z, w: 1. }
	}
}
// ------------------------------
impl Vec4 {
	pub fn new_position(x: f64, y: f64, z: f64) -> Vec4 {
		Vec4 { x: x, y: y, z: z, w: 1. }
	}
	pub fn new_direction(x: f64, y: f64, z: f64) -> Vec4 {
		Vec4 { x: x, y: y, z: z, w: 0. }
	}
	pub fn dot(&self, other: Vec4) -> f64 {
		self.x*other.x + self.y*other.y + self.z*other.z
	}
	pub fn cross(&self, other: Vec4) -> Vec4 {
		Vec4 { x: self.y * other.z - self.z * other.y,
			   y: self.z * other.x - self.x * other.z,
			   z: self.x * other.y - self.y * other.x,
			   w: 1. }
	}
	pub fn length(&self) -> f64 {
		self.length_sq().sqrt()
	}
	pub fn length_sq(&self) -> f64 {
		self.x*self.x + self.y*self.y + self.z*self.z
	}
	pub fn normalize(&mut self) -> Vec4 {
		let len = self.length();
		self.normalize_with_length(len)
	}
	pub fn normalize_with_length(&mut self, len: f64) -> Vec4 {
		self.x /= len;
		self.y /= len;
		self.z /= len;
		*self
	}
}
