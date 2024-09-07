use crate::{interval::Interval, vec3};

pub use vec3::Vec3 as Color;

pub fn write_color(pixel_colour: Color) -> String {
    let mut r = pixel_colour.x();
    let mut g = pixel_colour.y();
    let mut b = pixel_colour.z();

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    let intensity = Interval::new(0., 0.999);

    let rbyte = (255.999 * intensity.clamp(r)) as u32;
    let gbyte = (255.999 * intensity.clamp(g)) as u32;
    let bbyte = (255.999 * intensity.clamp(b)) as u32;

    format!("{rbyte} {gbyte} {bbyte}\n")
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}
