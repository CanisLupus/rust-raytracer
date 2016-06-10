#![allow(dead_code)]

mod bmp;
mod vec4;
mod mat4;
mod graphics;
mod lights;
mod structs;
mod scene;
mod scene_parser;

extern crate time;

use std::f64;
use graphics::*;
use lights::*;
use structs::*;
use vec4::*;
use scene::*;
use scene_parser::*;
use bmp::*;
use time::PreciseTime;

static EPS: f64 = 0.0001;

fn main() {
	// match parse_scene_file(String::from("scenes/scene4-specular.test")) {
	// match parse_scene_file(String::from("scenes/scene7.test")) {
	match parse_scene_file(String::from("scenes/scene6.test")) {
		Ok(scene) => {
			println!("Raytracing image...");
			let start = PreciseTime::now();
			let image = raytrace_scene(&scene);
			let end = PreciseTime::now();
			println!("Done after {} seconds.", start.to(end).num_milliseconds() as f64 / 1000.);

			image.write_to_file(scene.image_filename);
		}
		Err(err) => { println!("{}", err); }
	}
}

fn raytrace_scene(scene: &Scene) -> BmpImage {
	let mut image = BmpImage::new(scene.image_width, scene.image_height);

	for row in 0..scene.image_height {
		for col in 0..scene.image_width {
			let color = raytrace_from_pixel(row, col, &scene);
			image.set_pixel(row, col, color.r, color.g, color.b);
		}
	}
	image
}

fn raytrace_from_pixel(row: u32, col: u32, scene: &Scene) -> Color {
	let Camera { mut pos, fov_y, fov_x, axis_x, axis_y, axis_z, .. } = scene.camera;
	let half_width  = scene.image_width  as f64 / 2.;
	let half_height = scene.image_height as f64 / 2.;

	let alpha = f64::tan(fov_x/2.) * (col as f64 - half_width + 0.5)  / half_width;
	let beta  = f64::tan(fov_y/2.) * (half_height - row as f64 - 0.5) / half_height;
	let mut vec_through_pixel = (alpha * axis_x + beta * axis_y + axis_z).normalize();

	// TODO: make sure
	pos.w = 1.;
	vec_through_pixel.w = 0.;

	let ray = Ray { origin: pos, direction: vec_through_pixel };
	raytrace(ray, &scene, 0)
}

fn raytrace(ray: Ray, scene: &Scene, n_ray_bounces: u32) -> Color {
	if n_ray_bounces > scene.max_raytrace_depth {
		return Color::black();
	}

	let maybe_intersection = get_closest_intersection(&ray, &scene.graphics);

	if let Some((hit, hit_os, graphic, _)) = maybe_intersection {

		let mut normal_os = graphic.calc_normal_at(hit_os);
		// TODO: make sure
		normal_os.w = 0.;
		let mut normal = (*graphic.get_transposed_inverse_transform() * normal_os).normalize();
		// TODO: make sure
		normal.w = 0.;

		let material = graphic.get_material();
		let to_camera = (ray.origin - hit).normalize();
		let mut color = scene.ambient_color + material.ke;
		for light in &scene.lights {
			// TODO: re-enable
			if !is_shadowed_from_light(light, hit, &scene.graphics) {
				color = color + get_lighting_color(light, graphic, hit, to_camera, normal);
			}
		}

		// reflection
		let s = material.ks.r + material.ks.g + material.ks.b;
		if s > 0. {
			let reflection_direction = (ray.direction - 2. * ray.direction.dot(normal) * normal).normalize();
			let mut reflected_ray = Ray {
				direction: reflection_direction,
				origin: hit + EPS * reflection_direction,
			};
			// TODO: make sure
			reflected_ray.origin.w = 1.;
			reflected_ray.direction.w = 0.;

			let reflected_color = material.ks * raytrace(reflected_ray, scene, n_ray_bounces+1);
			color = color + reflected_color;
		}

		color
	} else {
		Color::black()
	}
}

fn get_closest_intersection<'a>(ray: &Ray, graphics: &'a Vec<Box<Graphic>>) -> Option<(Vec4, Vec4, &'a Box<Graphic>, f64)> {
	let mut closest_hit: Option<Vec4> = None;
	let mut closest_hit_os: Option<Vec4> = None;
	let mut closest_hit_distance: f64 = f64::INFINITY;
	let mut closest_graphic: Option<&Box<Graphic>> = None;

	for graphic in graphics {
		let ray_os = ray.transformed(graphic.get_inverse_transform());
		let maybe_hit_os = graphic.test_intersection(&ray_os);
		if let Some(mut hit_os) = maybe_hit_os {
			// TODO: make sure
			hit_os.w = 1.;
			let hit = *graphic.get_transform() * hit_os;

			let distance = (hit - ray.origin).length();	// TODO: optimize and use sqr distance
			if closest_hit_distance > distance {
				closest_hit_distance = distance;
				closest_hit = Some(hit);
				closest_hit_os = Some(hit_os);
				closest_graphic = Some(graphic);
			}
		}
	}

	if let Some(hit) = closest_hit {
		Some((hit, closest_hit_os.unwrap(), closest_graphic.unwrap(), closest_hit_distance))
	} else {
		None
	}
}

fn get_lighting_color(light: &Box<Light>, graphic: &Box<Graphic>, hit: Vec4, to_camera: Vec4, normal: Vec4) -> Color {
	let to_light = light.calc_to_light_direction(&hit);
	let to_light_dist = light.calc_to_light_distance(&hit);

	let ln = to_light.dot(normal);
	if ln > 0. {
		let lambert = ln * graphic.get_material().kd;
		let h = (to_light + to_camera).normalize();
		let normal_dot_h = normal.dot(h);
		let phong = f64::powf(if normal_dot_h > 0. { normal_dot_h } else { 0. }, graphic.get_material().shininess) * graphic.get_material().ks;
		let attenuation = light.calc_attenuation_at_distance(to_light_dist);
		(1. / attenuation) * light.get_color() * (lambert + phong)
	} else {
		Color::black()
	}
}

fn is_shadowed_from_light(light: &Box<Light>, point: Vec4, graphics: &Vec<Box<Graphic>>) -> bool {
	let to_light = light.calc_to_light_direction(&point);
	let to_light_dist = light.calc_to_light_distance(&point);
	let mut ray = Ray {
		origin: point + EPS * to_light,
		direction: to_light,
	};
	// TODO: make sure
	ray.origin.w = 1.;
	ray.direction.w = 0.;

	let maybe_intersection = get_closest_intersection(&ray, graphics);
	if let Some((_, _, _, distance)) = maybe_intersection {
		if distance < to_light_dist {
			return true;
		}
	}
	return false;
}
