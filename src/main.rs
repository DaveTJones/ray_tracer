#![allow(dead_code, unused_variables)]
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

use crate::vec3::Point3;
use camera::Camera;
use hittable_list::HittableList;
use sphere::Sphere;
use std::error::Error;

fn write_image() -> Result<(), Box<dyn Error>> {

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let cam = Camera::new(16. / 9., 400.);

    cam.render(&world)
}

fn main() {
    write_image().unwrap();
}
