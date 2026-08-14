mod point;
mod line;
mod polygon;

use image::{RgbImage, Rgb};
use point::Point;
use polygon::{Polygon, fill_polygons};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 450;

fn pts(coords: &[(i32, i32)]) -> Vec<Point> {
    coords.iter().map(|(x, y)| Point::new(*x, *y)).collect()
}

fn main() {
    let mut img = RgbImage::from_pixel(WIDTH, HEIGHT, Rgb([255, 255, 255]));

    let poly1 = Polygon::new(
        pts(&[(165,380),(185,360),(180,330),(207,345),(233,330),(230,360),(250,380),(220,385),(205,410),(193,383)]),
        Rgb([255, 0, 0]), Rgb([0, 0, 0]),
    );

    let poly2 = Polygon::new(
        pts(&[(321,335),(288,286),(339,251),(374,302)]),
        Rgb([0, 255, 0]), Rgb([0, 0, 0]),
    );

    let poly3 = Polygon::new(
        pts(&[(377,249),(411,197),(436,249)]),
        Rgb([0, 0, 255]), Rgb([0, 0, 0]),
    );

    let poly4 = Polygon::new(
        pts(&[(413,177),(448,159),(502,88),(553,53),(535,36),(676,37),(660,52),
              (750,145),(761,179),(672,192),(659,214),(615,214),(632,230),(580,230),
              (597,215),(552,214),(517,144),(466,180)]),
        Rgb([255, 165, 0]), Rgb([0, 0, 0]),
    );

    let poly5 = Polygon::new(
        pts(&[(682,175),(708,120),(735,148),(739,170)]),
        Rgb([255, 255, 255]), Rgb([0, 0, 0]), // color de relleno no se usa, es hueco
    );

    fill_polygons(&mut img, &[&poly1], Rgb([255, 0, 0]));
    fill_polygons(&mut img, &[&poly2], Rgb([0, 255, 0]));
    fill_polygons(&mut img, &[&poly3], Rgb([0, 0, 255]));
    fill_polygons(&mut img, &[&poly4, &poly5], Rgb([255, 165, 0])); // un solo color para toda la figura con hueco

    // Bordes al final para que no los tape el relleno
    poly1.draw_outline(&mut img);
    poly2.draw_outline(&mut img);
    poly3.draw_outline(&mut img);
    poly4.draw_outline(&mut img);
    poly5.draw_outline(&mut img);

    

    img.save("out.png").expect("No se pudo guardar la imagen");
    println!("out.png generado correctamente");
}