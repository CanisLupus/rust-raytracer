use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::SplitWhitespace;

use graphics::*;
use lights::*;
use structs::*;
use vec4::*;
use mat4::*;
use scene::*;

pub fn parse_scene_file(filepath: String) -> Result<Scene, io::Error> {
	let mut graphics: Vec<Box<Graphic>> = Vec::new();
	let mut lights: Vec<Box<Light>> = Vec::new();
	let mut camera: Camera = Default::default();
	let mut ambient_color: Color = Color::black();
	let mut max_raytrace_depth: u32 = 5;
	let mut image_width: u32 = 640;
	let mut image_height: u32 = 460;
	let mut image_filename: String = String::from("output.bmp");

	let mut attenuation: Attenuation = Default::default();
	let mut diffuse_color: Color = Color::black();
	let mut specular_color: Color = Color::black();
	let mut shininess: f64 = 0.;
	let mut emission_color: Color = Color::black();

	let mut vertices: Vec<Vec4> = Vec::new();

	let mut transformation_stack: Vec<Mat4> = Vec::new();
	transformation_stack.push(Mat4::id(1.));

	let f = try!(File::open(filepath));
	let reader = BufReader::new(f);

	for line in reader.lines() {
		let line_as_string = try!(line);
		let line = line_as_string.trim();
		if line.starts_with("#") || line.is_empty() {
			continue;
		}

		let mut tokens = line.split_whitespace();
		match tokens.next() {
			None => {}
			Some(command) => {
				match command {
					"break" => {
						break;
					},
					"size" => {
						image_width = parse_u32(&mut tokens);
						image_height = parse_u32(&mut tokens);
					},
					"maxdepth" => {
						max_raytrace_depth = parse_u32(&mut tokens);
					},
					"output" => {
						image_filename = tokens.collect::<Vec<&str>>().join(" ");
					},
					"camera" => {
						let pos     = Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:1. };
						let look_at	= Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:1. };
						let up	    = Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:0. }.normalize();	// note: w = 0
						let fov_y = parse_f64(&mut tokens);

						camera = Camera::new(
							pos,
							look_at,
							up,
							fov_y,
							image_width as f64,
							image_height as f64,
						);
					},
					"sphere" => {
						let transform: Mat4 = transformation_stack.last().unwrap().clone();
						let inverse_transform: Mat4 = transform.invert().unwrap();
						let transposed_inverse_transform: Mat4 = inverse_transform.transpose();

						let sphere = Sphere {
							pos: Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:1. },
							radius: parse_f64(&mut tokens),
							material: Material {
								ke: emission_color,
								kd: diffuse_color,
								ks: specular_color,
								shininess: shininess,
							},
							transform: transform,
							inverse_transform: inverse_transform,
							transposed_inverse_transform: transposed_inverse_transform,
						};
						graphics.push(Box::new(sphere));
					},
					"maxverts" => {
						vertices.reserve(parse_u32(&mut tokens) as usize);
					},
					"vertex" => {
						vertices.push(Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:1. });
					},
					"tri" => {
						let p1 = vertices[parse_u32(&mut tokens) as usize];
						let p2 = vertices[parse_u32(&mut tokens) as usize];
						let p3 = vertices[parse_u32(&mut tokens) as usize];

						let transform: Mat4 = transformation_stack.last().unwrap().clone();
						let inverse_transform: Mat4 = transform.invert().unwrap();
						let transposed_inverse_transform: Mat4 = inverse_transform.transpose();

						let triangle = Triangle {
							points: [p1, p2, p3],
							normal: (p3 - p2).cross(p1 - p2).normalize(),
							// normal: (p2 - p1).cross(p3 - p1).normalize(),
							material: Material {
								ke: emission_color,
								kd: diffuse_color,
								ks: specular_color,
								shininess: shininess,
							},
							transform: transform,
							inverse_transform: inverse_transform,
							transposed_inverse_transform: transposed_inverse_transform,
						};

						graphics.push(Box::new(triangle));
					},
					"translate" => {
						let matrix: Mat4 = Mat4::create_translation(parse_f64(&mut tokens), parse_f64(&mut tokens), parse_f64(&mut tokens));
						let top_matrix: Mat4 = transformation_stack.pop().unwrap();
						transformation_stack.push(top_matrix * matrix);
					},
					"rotate" => {
						let axis = Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:1. }.normalize();
						let matrix: Mat4 = Mat4::create_rotation(parse_f64(&mut tokens), axis);
						let top_matrix: Mat4 = transformation_stack.pop().unwrap();
						transformation_stack.push(top_matrix * matrix);
					},
					"scale" => {
						let matrix: Mat4 = Mat4::create_scale(parse_f64(&mut tokens), parse_f64(&mut tokens), parse_f64(&mut tokens));
						let top_matrix: Mat4 = transformation_stack.pop().unwrap();
						transformation_stack.push(top_matrix * matrix);
					},
					"pushTransform" => {
						let mat = transformation_stack.last().unwrap().clone();
						transformation_stack.push(mat);	// doubles the top element
					},
					"popTransform" => {
						match transformation_stack.pop() {
							Some(_) => {}
							None => { println!("Stack has no elements! Cannot pop."); }
						}
					},
					"directional" => {
						let directional_light = DirectionalLight {
							direction: Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:0. }.normalize(),	// note: w = 0
							color: Color { r:parse_f64(&mut tokens), g:parse_f64(&mut tokens), b:parse_f64(&mut tokens) },
						};
						lights.push(Box::new(directional_light));
					},
					"point" => {
						let point_light = PointLight {
							pos: Vec4 { x:parse_f64(&mut tokens), y:parse_f64(&mut tokens), z:parse_f64(&mut tokens), w:1. },
							color: Color { r:parse_f64(&mut tokens), g:parse_f64(&mut tokens), b:parse_f64(&mut tokens) },
							attenuation: attenuation.clone(),
						};
						lights.push(Box::new(point_light));
					},
					"attenuation" => {
						attenuation = Attenuation { constant:parse_f64(&mut tokens), linear:parse_f64(&mut tokens), quadratic:parse_f64(&mut tokens) };
					},
					"ambient" => {
						ambient_color = Color { r:parse_f64(&mut tokens), g:parse_f64(&mut tokens), b:parse_f64(&mut tokens) };
					},
					"diffuse" => {
						diffuse_color = Color { r:parse_f64(&mut tokens), g:parse_f64(&mut tokens), b:parse_f64(&mut tokens) };
					},
					"specular" => {
						specular_color = Color { r:parse_f64(&mut tokens), g:parse_f64(&mut tokens), b:parse_f64(&mut tokens) };
					},
					"shininess" => {
						shininess = parse_f64(&mut tokens);
					},
					"emission" => {
						emission_color = Color { r:parse_f64(&mut tokens), g:parse_f64(&mut tokens), b:parse_f64(&mut tokens) };
					},
					_ => {
						println!("Unrecognized command {}!", command);
					}
				}
			}
		}
	}

	Ok(Scene {
		graphics: graphics,
		lights: lights,
		camera: camera,
		ambient_color: ambient_color,
		max_raytrace_depth: max_raytrace_depth,
		image_width: image_width,
		image_height: image_height,
		image_filename: image_filename,
	})
}

fn parse_u32(tokens: &mut SplitWhitespace) -> u32 {
	tokens.next().unwrap().parse::<u32>().unwrap()
}
fn parse_f64(tokens: &mut SplitWhitespace) -> f64 {
	tokens.next().unwrap().parse::<f64>().unwrap()
}
