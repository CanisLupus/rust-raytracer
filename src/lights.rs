use std::f64;
use structs::*;
use vec4::*;

// ---------- Light ----------

pub trait Light {
	fn get_color(&self) -> Color;
	fn calc_attenuation_at_distance(&self, distance: f64) -> f64;
	fn calc_to_light_direction(&self, point: &Vec4) -> Vec4;
	fn calc_to_light_distance(&self, point: &Vec4) -> f64;
}

// ---------- PointLight ----------

#[derive(Debug)]
pub struct PointLight {
	pub pos: Vec4,
	pub color: Color,
	pub attenuation: Attenuation,
}

impl Light for PointLight {
	fn get_color(&self) -> Color {
		self.color
	}

	fn calc_attenuation_at_distance(&self, distance: f64) -> f64 {
		self.attenuation.constant
		+ self.attenuation.linear * distance
		+ self.attenuation.quadratic * distance * distance
	}

	fn calc_to_light_direction(&self, point: &Vec4) -> Vec4 {
		(self.pos - *point).normalize()
	}

	fn calc_to_light_distance(&self, point: &Vec4) -> f64 {
		(self.pos - *point).length()
	}
}

// ---------- DirectionalLight ----------

#[derive(Debug)]
pub struct DirectionalLight {
	pub direction: Vec4,
	pub color: Color,
}

impl Light for DirectionalLight {
	fn get_color(&self) -> Color {
		self.color
	}

	fn calc_attenuation_at_distance(&self, _distance: f64) -> f64 {
		1.
	}

	fn calc_to_light_direction(&self, _point: &Vec4) -> Vec4 {
		self.direction
	}

	fn calc_to_light_distance(&self, _point: &Vec4) -> f64 {
		f64::INFINITY
	}
}
