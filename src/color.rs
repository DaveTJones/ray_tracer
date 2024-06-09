use crate::vec3;

pub use vec3::Vec3 as Color;

pub fn write_color(pixel_colour: Color) -> String {
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();

    let rbyte = (255.999 * r) as u32;
    let gbyte = (255.999 * g) as u32;
    let bbyte = (255.999 * b) as u32;

    // format!("{} {} {}\n", ir as u32, ig as u32, ib as u32).as_bytes()

    format!("{rbyte} {gbyte} {bbyte}\n")
}
