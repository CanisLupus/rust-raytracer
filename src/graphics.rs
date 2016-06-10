use structs::*;
use vec4::*;
use mat4::*;

// ---------- Graphic ----------

pub trait Graphic {
	fn get_material(&self) -> &Material;
	fn get_transform(&self) -> &Mat4;
	fn get_inverse_transform(&self) -> &Mat4;
	fn get_transposed_inverse_transform(&self) -> &Mat4;
	fn test_intersection(&self, ray: &Ray) -> Option<Vec4>;
	fn calc_normal_at(&self, point: Vec4) -> Vec4;
}

fn test_plane_intersection(ray: &Ray, pos: Vec4, normal: Vec4) -> Option<Vec4> {
	let t = normal.dot(pos - ray.origin) / normal.dot(ray.direction);
	if t <= 0. {
		None
	} else {
		Some(ray.origin + t * ray.direction)
	}
}

fn is_inside(point: Vec4, origin: Vec4, p1: Vec4, p2: Vec4) -> bool {
	let normal = (p2 - origin).cross(p1 - origin).normalize();
	let d = (-origin).dot(normal);
	return point.dot(normal) + d >= 0.;
}

// ---------- Sphere ----------

#[derive(Debug)]
pub struct Sphere {
	pub pos: Vec4,
	pub radius: f64,
	pub material: Material,
	pub transform: Mat4,
	pub inverse_transform: Mat4,
	pub transposed_inverse_transform: Mat4,
}

impl Graphic for Sphere {
	fn get_material(&self) -> &Material {
		&self.material
	}
	fn get_transform(&self) -> &Mat4 {
		&self.transform
	}
	fn get_inverse_transform(&self) -> &Mat4 {
		&self.inverse_transform
	}
	fn get_transposed_inverse_transform(&self) -> &Mat4 {
		&self.transposed_inverse_transform
	}

	fn test_intersection(&self, ray: &Ray) -> Option<Vec4> {
		let origin_to_center = self.pos - ray.origin;
		let b = origin_to_center.dot(ray.direction);

		let det = b*b - origin_to_center.length_sq() + self.radius*self.radius;
		if det < 0. {
			return None;
		}

		let squared_det = det.sqrt();
		let t0 = b - squared_det;
		if t0 > 1e-3 {
			return Some(ray.origin + t0 * ray.direction);
		}

		let t1 = b + squared_det;
		if t1 > 1e-3 {
			return Some(ray.origin + t1 * ray.direction);
		}

		return None;
	}

	fn calc_normal_at(&self, point: Vec4) -> Vec4 {
		let mut to_point = point - self.pos;
		to_point.normalize()
	}
}

// ---------- Plane ----------

#[derive(Debug)]
pub struct Plane {
	pub pos: Vec4,
	pub normal: Vec4,
	pub material: Material,
	pub transform: Mat4,
	pub inverse_transform: Mat4,
	pub transposed_inverse_transform: Mat4,
}

impl Graphic for Plane {
	fn get_material(&self) -> &Material {
		&self.material
	}
	fn get_transform(&self) -> &Mat4 {
		&self.transform
	}
	fn get_inverse_transform(&self) -> &Mat4 {
		&self.inverse_transform
	}
	fn get_transposed_inverse_transform(&self) -> &Mat4 {
		&self.transposed_inverse_transform
	}

	fn test_intersection(&self, ray: &Ray) -> Option<Vec4> {
		test_plane_intersection(ray, self.pos, self.normal)
	}

	fn calc_normal_at(&self, _point: Vec4) -> Vec4 {
		self.normal
	}
}

// ---------- Triangle ----------

#[derive(Debug)]
pub struct Triangle {
	pub points: [Vec4; 3],
	pub normal: Vec4,
	pub material: Material,
	pub transform: Mat4,
	pub inverse_transform: Mat4,
	pub transposed_inverse_transform: Mat4,
}

impl Graphic for Triangle {
	fn get_material(&self) -> &Material {
		&self.material
	}
	fn get_transform(&self) -> &Mat4 {
		&self.transform
	}
	fn get_inverse_transform(&self) -> &Mat4 {
		&self.inverse_transform
	}
	fn get_transposed_inverse_transform(&self) -> &Mat4 {
		&self.transposed_inverse_transform
	}

	fn test_intersection(&self, ray: &Ray) -> Option<Vec4> {
		if let Some(hit) = test_plane_intersection(ray, self.points[0], self.normal) {
			// check if intersection is inside triangle; i.e.: if all calculations return the same side (side is either true or false)
			let inside: bool = is_inside(hit, self.points[0], self.points[1], ray.origin);
			if inside == is_inside(hit, self.points[1], self.points[2], ray.origin)
			&& inside == is_inside(hit, self.points[2], self.points[0], ray.origin) {
				return Some(hit);
			 }
		}
		return None;
	}

	fn calc_normal_at(&self, _point: Vec4) -> Vec4 {
		self.normal
	}
}
