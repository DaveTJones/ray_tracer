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
use color::Color;
use hittable::HitRecord;
use hittable_list::HittableList;
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use std::error::Error;

fn write_image() -> Result<(), Box<dyn Error>> {
    // World

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let cam = Camera::new(16. / 9., 400.);

    cam.render(&world)
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();

    if world.hit(r, Interval::new(0., f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().to_unit_vector();

    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + (a) * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    write_image().unwrap();
}
