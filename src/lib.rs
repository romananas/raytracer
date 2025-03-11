#![allow(dead_code)]

mod camera;
pub mod color;
pub mod common;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
pub mod vec3;
mod quad;
mod cube;
mod cylinder;
mod disk;

pub use quad::Quad;
// pub use rayon::prelude::*;

pub use camera::Camera;
pub use color::Color;
pub use hittable::Hittable;
pub use hittable_list::HittableList;
pub use material::{Dielectric, Lambertian, Metal};
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Point3,Vec3};
pub use cube::Cube;
pub use cylinder::Cylinder;