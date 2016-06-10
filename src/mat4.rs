use std::ops::{Add,Mul};
use vec4::*;

#[derive(Debug,Copy,Clone)]
pub struct Mat4 {
	pub data: [[f64; 4]; 4]
}
// ------------------------------
// impl<'a> Index<usize> for Mat4 {
// 	type Output = &'a [f64; 4];
// 	fn index(&'a self, i: usize) -> &'a [f64; 4] {
// 		&self.data[i]
// 	}
// }
// ------------------------------
impl Add for Mat4 {
	type Output = Mat4;
	fn add(self, o: Mat4) -> Mat4 {
		let mut data = [[0.0; 4]; 4];
		for row in 0..4 {
			for col in 0..4 {
				data[row][col] = self.data[row][col] + o.data[row][col];
			}
		}
		Mat4 { data: data }
	}
}
// ------------------------------
impl Mul for Mat4 {
	type Output = Mat4;
	fn mul(self, o: Mat4) -> Mat4 {
		let mut data = [[0.0; 4]; 4];
		for row in 0..4 {
			for col in 0..4 {
				let mut sum: f64 = 0.0;
				for k in 0..4 {
					sum += self.data[row][k] * o.data[k][col];
				}
				data[row][col] = sum;
			}
		}

		Mat4 { data: data }
	}
}
impl Mul<Vec4> for Mat4 {
	type Output = Vec4;
	fn mul(self, o: Vec4) -> Vec4 {
		Vec4 {
			x: o.x * self.data[0][0] + o.y * self.data[0][1] + o.z * self.data[0][2] + o.w * self.data[0][3],
			y: o.x * self.data[1][0] + o.y * self.data[1][1] + o.z * self.data[1][2] + o.w * self.data[1][3],
			z: o.x * self.data[2][0] + o.y * self.data[2][1] + o.z * self.data[2][2] + o.w * self.data[2][3],
			w: o.x * self.data[3][0] + o.y * self.data[3][1] + o.z * self.data[3][2] + o.w * self.data[3][3],
		}
	}
}
impl Mul<f64> for Mat4 {
	type Output = Mat4;
	fn mul(self, o: f64) -> Mat4 {
		let mut data = [[0.0; 4]; 4];
		for row in 0..4 {
			for col in 0..4 {
				data[row][col] = self.data[row][col] * o;
			}
		}

		Mat4 { data: data }
	}
}
impl Mul<Mat4> for f64 {
	type Output = Mat4;
	fn mul(self, o: Mat4) -> Mat4 {
		o * self
	}
}
// // ------------------------------
impl Mat4 {
	pub fn id(val: f64) -> Mat4 {
		Mat4 {
			data: [
				[val, 0.,  0.,  0. ],
				[0.,  val, 0.,  0. ],
				[0.,  0.,  val, 0. ],
				[0.,  0.,  0.,  val],
			]
		}
	}

	pub fn create_translation(tx: f64, ty: f64, tz: f64) -> Mat4 {
		Mat4 {
			data: [
				[1., 0., 0., tx],
				[0., 1., 0., ty],
				[0., 0., 1., tz],
				[0., 0., 0., 1.],
			]
		}
	}

	pub fn create_scale(sx: f64, sy: f64, sz: f64) -> Mat4 {
		Mat4 {
			data: [
				[sx, 0., 0., 0.],
				[0., sy, 0., 0.],
				[0., 0., sz, 0.],
				[0., 0., 0., 1.],
			]
		}
	}

	pub fn create_rotation(degrees: f64, Vec4 { x, y, z, .. }: Vec4) -> Mat4 {
		let m1 = Mat4 {
			data: [
				[x*x, x*y, x*z, 0.],
				[y*x, y*y, y*z, 0.],
				[z*x, z*y, z*z, 0.],
				[0.,  0.,  0.,  0.],
			]
		};
		let m2 = Mat4 {
			data: [
				[ 0., -z,   y,  0.],
				[ z,   0., -x,  0.],
				[-y,   x,   0., 0.],
				[ 0.,  0.,  0., 0.],
			]
		};

		let mut one = Mat4::id(1.);
		one.data[3][3] = 0.;
		let radians = degrees.to_radians();
		let mut res = f64::cos(radians) * one + (1. - f64::cos(radians)) * m1 + f64::sin(radians) * m2;
		res.data[3][3] = 1.;
		res
	}

	pub fn transpose(&self) -> Mat4 {
		let mut data = [[0.0; 4]; 4];
		for row in 0..4 {
			for col in 0..4 {
				data[row][col] = self.data[col][row];
			}
		}

		Mat4 { data: data }
	}

	pub fn invert(&self) -> Result<Mat4,()> {
		let mut inv = [[0.0; 4]; 4];

		let m = &self.data;

		inv[0][0] = m[1][1] * m[2][2] * m[3][3] -
					m[1][1] * m[2][3] * m[3][2] -
					m[2][1] * m[1][2] * m[3][3] +
					m[2][1] * m[1][3] * m[3][2] +
					m[3][1] * m[1][2] * m[2][3] -
					m[3][1] * m[1][3] * m[2][2];

		inv[1][0] = -m[1][0] * m[2][2] * m[3][3] +
					m[1][0] * m[2][3] * m[3][2] +
					m[2][0] * m[1][2] * m[3][3] -
					m[2][0] * m[1][3] * m[3][2] -
					m[3][0] * m[1][2] * m[2][3] +
					m[3][0] * m[1][3] * m[2][2];

		inv[2][0] = m[1][0] * m[2][1] * m[3][3] -
					m[1][0] * m[2][3] * m[3][1] -
					m[2][0] * m[1][1] * m[3][3] +
					m[2][0] * m[1][3] * m[3][1] +
					m[3][0] * m[1][1] * m[2][3] -
					m[3][0] * m[1][3] * m[2][1];

		inv[3][0] = -m[1][0] * m[2][1] * m[3][2] +
					m[1][0] * m[2][2] * m[3][1] +
					m[2][0] * m[1][1] * m[3][2] -
					m[2][0] * m[1][2] * m[3][1] -
					m[3][0] * m[1][1] * m[2][2] +
					m[3][0] * m[1][2] * m[2][1];

		inv[0][1] = -m[0][1] * m[2][2] * m[3][3] +
					m[0][1] * m[2][3] * m[3][2] +
					m[2][1] * m[0][2] * m[3][3] -
					m[2][1] * m[0][3] * m[3][2] -
					m[3][1] * m[0][2] * m[2][3] +
					m[3][1] * m[0][3] * m[2][2];

		inv[1][1] = m[0][0] * m[2][2] * m[3][3] -
					m[0][0] * m[2][3] * m[3][2] -
					m[2][0] * m[0][2] * m[3][3] +
					m[2][0] * m[0][3] * m[3][2] +
					m[3][0] * m[0][2] * m[2][3] -
					m[3][0] * m[0][3] * m[2][2];

		inv[2][1] = -m[0][0] * m[2][1] * m[3][3] +
					m[0][0] * m[2][3] * m[3][1] +
					m[2][0] * m[0][1] * m[3][3] -
					m[2][0] * m[0][3] * m[3][1] -
					m[3][0] * m[0][1] * m[2][3] +
					m[3][0] * m[0][3] * m[2][1];

		inv[3][1] = m[0][0] * m[2][1] * m[3][2] -
					m[0][0] * m[2][2] * m[3][1] -
					m[2][0] * m[0][1] * m[3][2] +
					m[2][0] * m[0][2] * m[3][1] +
					m[3][0] * m[0][1] * m[2][2] -
					m[3][0] * m[0][2] * m[2][1];

		inv[0][2] = m[0][1] * m[1][2] * m[3][3] -
					m[0][1] * m[1][3] * m[3][2] -
					m[1][1] * m[0][2] * m[3][3] +
					m[1][1] * m[0][3] * m[3][2] +
					m[3][1] * m[0][2] * m[1][3] -
					m[3][1] * m[0][3] * m[1][2];

		inv[1][2] = -m[0][0] * m[1][2] * m[3][3] +
					m[0][0] * m[1][3] * m[3][2] +
					m[1][0] * m[0][2] * m[3][3] -
					m[1][0] * m[0][3] * m[3][2] -
					m[3][0] * m[0][2] * m[1][3] +
					m[3][0] * m[0][3] * m[1][2];

		inv[2][2] = m[0][0] * m[1][1] * m[3][3] -
					m[0][0] * m[1][3] * m[3][1] -
					m[1][0] * m[0][1] * m[3][3] +
					m[1][0] * m[0][3] * m[3][1] +
					m[3][0] * m[0][1] * m[1][3] -
					m[3][0] * m[0][3] * m[1][1];

		inv[3][2] = -m[0][0] * m[1][1] * m[3][2] +
					m[0][0] * m[1][2] * m[3][1] +
					m[1][0] * m[0][1] * m[3][2] -
					m[1][0] * m[0][2] * m[3][1] -
					m[3][0] * m[0][1] * m[1][2] +
					m[3][0] * m[0][2] * m[1][1];

		inv[0][3] = -m[0][1] * m[1][2] * m[2][3] +
					m[0][1] * m[1][3] * m[2][2] +
					m[1][1] * m[0][2] * m[2][3] -
					m[1][1] * m[0][3] * m[2][2] -
					m[2][1] * m[0][2] * m[1][3] +
					m[2][1] * m[0][3] * m[1][2];

		inv[1][3] = m[0][0] * m[1][2] * m[2][3] -
					m[0][0] * m[1][3] * m[2][2] -
					m[1][0] * m[0][2] * m[2][3] +
					m[1][0] * m[0][3] * m[2][2] +
					m[2][0] * m[0][2] * m[1][3] -
					m[2][0] * m[0][3] * m[1][2];

		inv[2][3] = -m[0][0] * m[1][1] * m[2][3] +
					m[0][0] * m[1][3] * m[2][1] +
					m[1][0] * m[0][1] * m[2][3] -
					m[1][0] * m[0][3] * m[2][1] -
					m[2][0] * m[0][1] * m[1][3] +
					m[2][0] * m[0][3] * m[1][1];

		inv[3][3] = m[0][0] * m[1][1] * m[2][2] -
					m[0][0] * m[1][2] * m[2][1] -
					m[1][0] * m[0][1] * m[2][2] +
					m[1][0] * m[0][2] * m[2][1] +
					m[2][0] * m[0][1] * m[1][2] -
					m[2][0] * m[0][2] * m[1][1];

		let pre_det: f64 = m[0][0] * inv[0][0]
						 + m[0][1] * inv[1][0]
						 + m[0][2] * inv[2][0]
						 + m[0][3] * inv[3][0];

		if pre_det == 0.0 {
			return Err(());
		}

		let det = 1.0 / pre_det;

		for row in 0..4 {
			for col in 0..4 {
				inv[row][col] *= det;
			}
		}

		return Ok(Mat4 { data: inv });
	}
}
