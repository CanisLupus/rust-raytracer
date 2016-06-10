rust-raytracer
=================

#### Description

A simple graphics ray tracer coded in Rust. You can use it as a learning tool or as guidance when creating a similar ray tracer :)

The ray tracer is compatible with the online course "CS184.1x Foundations of Computer Graphics" at edX, supporting the scene setup file format used there (description of the scene to be traced). I completed the course with a C++ ray tracer, but due to respect for the teachers of the course I'll not post that code, as C++ is the "official" course programming language. If you do use this repository to help you with that course, DO learn from it and DON'T simply copy work.

#### Features

- Sphere, Triangle and Plane intersection routines (extendable to general convex polygons)
- Point and directional lights (as well as ambient light)
- Matrix transformations (translate, scale, rotate) for collisions in object space
- Shadows and reflections
- Phong reflection model (plus emission value)
- Perspective camera with field-of-view
- Support for scene description files (containing the scene setup)

Does not include refraction, soft shadows, cone lights, texture support, more complex surfaces (think cylinders and cones) or fancy global illumination techniques. It is also very simple regarding optimization. I took care in avoiding unnecessary computations, but didn't implement any optimizations that are essential for a fast ray tracer, such as bounding boxes or other partition schemes.

#### Compilation / Execution

Fast:

	cargo run --release

Debug:

	cargo run

#### Known issues

- The scene configuration files use PNG as the image output format, but this ray tracer saves the images as BMP (but still with the .png extension).
- Generating the image for scene7.test probably takes a few hours, as the raytracer is simply not optimized enough.
