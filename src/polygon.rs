use image::{RgbImage, Rgb};
use crate::point::Point;
use crate::line::draw_line;

pub struct Polygon {
    pub points: Vec<Point>,
    pub fill_color: Rgb<u8>,
    pub line_color: Rgb<u8>,
}

impl Polygon {
    pub fn new(points: Vec<Point>, fill_color: Rgb<u8>, line_color: Rgb<u8>) -> Self {
        Polygon { points, fill_color, line_color }
    }

    pub fn draw_outline(&self, img: &mut RgbImage) {
        let n = self.points.len();
        for i in 0..n {
            let p0 = self.points[i];
            let p1 = self.points[(i + 1) % n];
            draw_line(img, p0, p1, self.line_color);
        }
    }
}

// Rellena un conjunto de polígonos usando regla par-impar.
// Si pasas varios polígonos juntos (ej: figura + hueco),
// el hueco queda automáticamente sin pintar.
pub fn fill_polygons(img: &mut RgbImage, polygons: &[&Polygon]) {
    // Recolectamos TODAS las aristas de TODOS los polígonos juntos
    let mut edges: Vec<(Point, Point, Rgb<u8>)> = Vec::new();
    for poly in polygons {
        let n = poly.points.len();
        for i in 0..n {
            let p0 = poly.points[i];
            let p1 = poly.points[(i + 1) % n];
            edges.push((p0, p1, poly.fill_color));
        }
    }

    let min_y = edges.iter().map(|(p0, p1, _)| p0.y.min(p1.y)).min().unwrap_or(0);
    let max_y = edges.iter().map(|(p0, p1, _)| p0.y.max(p1.y)).max().unwrap_or(0);

    for y in min_y..=max_y {
        // Buscamos intersecciones de la scanline y con cada arista
        let mut intersections: Vec<(f64, Rgb<u8>)> = Vec::new();

        for (p0, p1, color) in &edges {
            let (y0, y1) = (p0.y, p1.y);
            if y0 == y1 { continue; } // arista horizontal, se ignora

            // La scanline debe cruzar la arista (regla: incluye un extremo, excluye el otro)
            if (y >= y0 && y < y1) || (y >= y1 && y < y0) {
                let t = (y - y0) as f64 / (y1 - y0) as f64;
                let x = p0.x as f64 + t * (p1.x - p0.x) as f64;
                intersections.push((x, *color));
            }
        }

        // Ordenamos de izquierda a derecha
        intersections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Pintamos entre pares: (0-1) pintado, (2-3) pintado, etc.
        let mut i = 0;
        while i + 1 < intersections.len() {
            let (x_start, color) = intersections[i];
            let (x_end, _) = intersections[i + 1];

            let xs = x_start.round() as i32;
            let xe = x_end.round() as i32;

            for x in xs..=xe {
                if x >= 0 && y >= 0 && (x as u32) < img.width() && (y as u32) < img.height() {
                    img.put_pixel(x as u32, y as u32, color);
                }
            }
            i += 2;
        }
    }
}