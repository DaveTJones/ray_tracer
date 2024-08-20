#![allow(dead_code, unused_variables)]
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod ray;
pub mod sphere;
pub mod vec3;

use color::Color;
use hittable::HitRecord;
use hittable_list::HittableList;
use indicatif::{ProgressBar, ProgressStyle};
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use std::error::Error;
use std::{fs::File, io::Write};
use vec3::{Point3, Vec3};

fn write_image() -> Result<(), Box<dyn Error>> {
    let mut file = File::create("image.ppm")?;

    // Image

    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width = 4000.;

    // Calculate the image height, and ensure that it's at least 1.
    let image_height = (image_width / aspect_ratio).floor().max(1.);

    // World

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    //Camera

    let focal_length = 1.;

    let viewport_height = 2.;

    let viewport_width = viewport_height * (image_width / image_height);

    let camera_centre = vec3::Point3::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vert viewport edges.

    let viewport_u = Vec3::new(viewport_width, 0., 0.);

    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and verical delta vectors from pixel to pixel.

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate the location of the upper left pixel.

    let viewport_upper_left =
        camera_centre - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    let image_width: u64 = image_width as u64;
    let image_height: u64 = image_height as u64;

    let pb = ProgressBar::new(image_width * image_height);
    pb.set_style(ProgressStyle::with_template("{msg}")?);

    file.write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            let ray_direction = pixel_center - camera_centre;

            let r = Ray::new(camera_centre, ray_direction);

            let pixel_color = ray_color(&r, &world);

            let color = color::write_color(pixel_color);

            file.write(color.as_bytes())?;

            pb.inc(1);
        }
    }

    file.flush()?;

    pb.finish_with_message("Image created");

    Ok(())
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();

    // let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, &r);
    if world.hit(r, Interval::new(0., f64::INFINITY), &mut rec) {
        // println!("{:?}", rec.normal);
        //TODO: figure out why this hit isnt modifying the valus of normal away from default
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();

    let a = 0.5 * (unit_direction.y() + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + (a) * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    write_image().unwrap();
}
