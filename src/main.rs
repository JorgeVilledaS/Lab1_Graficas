use image::{RgbImage, Rgb};
mod point;
use point::Point;
mod line;
mod polygon;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 450;

fn main() {
    let img = RgbImage::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));
    img.save("out.png").expect("No se pudo guardar la imagen");
}