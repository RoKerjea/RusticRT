pub mod tuple;
pub mod color;
pub mod canvas;
pub mod ray;
pub mod sphere;
pub mod plane;
pub mod camera;
pub mod body;
pub mod lights;
pub mod world;
pub mod intersections;
pub mod computed_intersection;
pub mod material;

pub mod matrix;

#[macro_use]
mod fuzzy_eq;

type F = f64;

pub const EPSILON: f64 = 0.00001;