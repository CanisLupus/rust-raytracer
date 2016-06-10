use std::ops::{Add,Sub,Mul};
use vec4::*;
use mat4::*;

// ---------- Material ----------

#[derive(Debug)]
pub struct Material {
	pub ke: Color,
	pub kd: Color,
	pub ks: Color,
	pub shininess: f64,
}

// ---------- Ray ----------

#[derive(Debug)]
pub struct Ray {
	pub origin: Vec4,
	pub direction: Vec4,
}

impl Ray {
	pub fn transformed(&self, transform: &Mat4) -> Ray {
		let mut ray = Ray {
			origin: *transform * self.origin,
			direction: (*transform * self.direction).normalize(),
		};
		// TODO: make sure
		ray.origin.w = 1.;
		ray.direction.w = 0.;
		ray
	}
}

// ---------- Color ----------

#[derive(Debug,Copy,Clone,Default)]
pub struct Color {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl Color {
	pub fn black() -> Color {
		Color { r:0., g:0., b:0. }
	}
}

impl Add for Color {
	type Output = Color;
	fn add(self, o: Color) -> Color {
		Color { r: self.r + o.r, g: self.g + o.g, b: self.b + o.b }
	}
}

impl Sub for Color {
	type Output = Color;
	fn sub(self, o: Color) -> Color {
		Color { r: self.r - o.r, g: self.g - o.g, b: self.b - o.b }
	}
}

impl Mul for Color {
	type Output = Color;
	fn mul(self, o: Color) -> Color {
		Color { r: self.r * o.r, g: self.g * o.g, b: self.b * o.b }
	}
}

impl Mul<f64> for Color {
	type Output = Color;
	fn mul(self, o: f64) -> Color {
		Color { r: self.r * o, g: self.g * o, b: self.b * o }
	}
}

impl Mul<Color> for f64 {
	type Output = Color;
	fn mul(self, o: Color) -> Color {
		Color { r: self * o.r, g: self * o.g, b: self * o.b }
	}
}

// ---------- Attenuation ----------

#[derive(Debug,Clone)]
pub struct Attenuation {
	pub constant: f64,
	pub linear: f64,
	pub quadratic: f64,
}

impl Default for Attenuation {
    fn default() -> Attenuation {
		Attenuation { constant: 1., linear: 0., quadratic: 0. }
	}
}
