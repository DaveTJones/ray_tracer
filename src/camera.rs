use std::{error::Error, fs::File, io::Write};
use vec3::{Point3, Vec3};

use crate::{
    color::{self, Color},
    hittable::HitRecord,
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    vec3,
};
use indicatif::{ProgressBar, ProgressStyle};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u64,
    pub samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
    center: Point3,
    image_height: u64,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: f64) -> Self {
        let samples_per_pixel = 100;

        let max_depth = 10;
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = (image_width / aspect_ratio).floor().max(1.);

        let pixel_samples_scale = 1. / samples_per_pixel as f64;

        //Camera
        let focal_length = 1.;

        let viewport_height = 2.;

        let viewport_width = viewport_height * (image_width / image_height);

        let center = vec3::Point3::new(0., 0., 0.);

        // Calculate the vectors across the horizontal and down the vert viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0., 0.);

        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and verical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Render
        let image_width: u64 = image_width as u64;
        let image_height: u64 = image_height as u64;

        Self {
            aspect_ratio,
            image_width,
            pixel_samples_scale,
            max_depth,
            samples_per_pixel,
            center,
            image_height,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &HittableList) -> Result<(), Box<dyn Error>> {
        let mut file = File::create("image.ppm")?;
        let pb = ProgressBar::new(self.image_width * self.image_height);
        pb.set_style(ProgressStyle::with_template("{msg}")?);

        file.write(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0., 0., 0.);

                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                let color = color::write_color(pixel_color * self.pixel_samples_scale);

                file.write(color.as_bytes())?;

                pb.inc(1);
            }
        }

        file.flush()?;

        pb.finish_with_message("Image created");

        Ok(())
    }

    fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
        if depth < 0 {
            return Color::new(0., 0., 0.);
        }

        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = Vec3::random_on_hemisphere(&rec.normal) + Vec3::random_unit_vector();
            return 0.1 * Self::ray_color(&Ray::new(rec.p, direction), depth - 1, &world);
            // return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction().to_unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + (a) * Color::new(0.5, 0.7, 1.0)
    }

    fn get_ray(&self, i: u64, j: u64) -> Ray {
        let offset = Self::sample_square();

        let pixel_sample = self.pixel00_loc
            + (i as f64 + offset.x()) * self.pixel_delta_u
            + (j as f64 + offset.y()) * self.pixel_delta_v;

        let ray_origin = self.center;

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // TODO: there is some duplication between this code and the random method on the vec3 type.
        // find a way to dedupe
        let (x, y) = rand::random();
        Vec3::new(x, y, 0.0)
    }
}
