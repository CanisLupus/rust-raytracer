use graphics::*;
use lights::*;
use structs::*;
use vec4::*;

// ---------- Scene ----------

#[derive(Default)]
pub struct Scene {
	pub graphics: Vec<Box<Graphic>>,
	pub lights: Vec<Box<Light>>,
	pub camera: Camera,
	pub ambient_color: Color,
	pub max_raytrace_depth: u32,

	pub image_width: u32,
	pub image_height: u32,
	pub image_filename: String,
}

// ---------- Camera ----------

#[derive(Debug,Default)]
pub struct Camera {
	pub pos: Vec4,		// position of the camera in space
	pub look_at: Vec4,	// position the camera is looking at
	pub up: Vec4,		// vector indicating which way is up (rotates the camera view)
	pub fov_y: f64,
	pub fov_x: f64,
	pub axis_x: Vec4,
	pub axis_y: Vec4,
	pub axis_z: Vec4,
}

impl Camera {
	pub fn new(pos: Vec4, look_at: Vec4, up: Vec4, fov_y: f64, view_width: f64, view_height: f64) -> Camera {
		let axis_z = (look_at - pos).normalize();
		let axis_x = axis_z.clone().normalize().cross(up).normalize();
		let axis_y = axis_x.cross(axis_z);
		let fov_y_rad = fov_y.to_radians();

		Camera {
			pos: pos,
			look_at: look_at,
			up: up,
			fov_y: fov_y_rad,
			fov_x: Camera::calc_fov_x(fov_y_rad, view_width, view_height),
			axis_x: axis_x,
			axis_y: axis_y,
			axis_z: axis_z,
		}
	}

	pub fn calc_fov_x(fov_y: f64, view_width: f64, view_height: f64) -> f64 {
		2. * f64::atan(f64::tan(fov_y/2.) * view_width / view_height)
	}
}
