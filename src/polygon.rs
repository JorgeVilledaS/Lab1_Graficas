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
//
pub fn fill_polygons(img: &mut RgbImage, polygons: &[&Polygon], color: Rgb<u8>) {
    // Recolectamos TODAS las aristas de TODOS los polígonos juntos
    let mut edges: Vec<(Point, Point)> = Vec::new();
    for poly in polygons {
        let n = poly.points.len();
        for i in 0..n {
            let p0 = poly.points[i];
            let p1 = poly.points[(i + 1) % n];
            edges.push((p0, p1));
        }
    }

    let min_y = edges.iter().map(|(p0, p1)| p0.y.min(p1.y)).min().unwrap_or(0);
    let max_y = edges.iter().map(|(p0, p1)| p0.y.max(p1.y)).max().unwrap_or(0);

    for y in min_y..=max_y {
        // Muestreamos en el CENTRO del pixel (y + 0.5) para evitar
        // ambigüedad cuando un vértice cae justo en una coordenada entera
        // (picos/valles duplicando intersecciones).
        let yc = y as f64 + 0.5;

        let mut xs_inter: Vec<f64> = Vec::new();

        for (p0, p1) in &edges {
            let (y0, y1) = (p0.y as f64, p1.y as f64);
            if y0 == y1 { continue; } // arista horizontal, se ignora

            if (yc >= y0 && yc < y1) || (yc >= y1 && yc < y0) {
                let t = (yc - y0) / (y1 - y0);
                let x = p0.x as f64 + t * (p1.x - p0.x) as f64;
                xs_inter.push(x);
            }
        }

        // Ordenamos de izquierda a derecha
        xs_inter.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Pintamos entre pares: (0-1) pintado, (2-3) pintado, etc.
        // TODOS los pares usan el mismo `color` (un solo parámetro).
        let mut i = 0;
        while i + 1 < xs_inter.len() {
            let x_start = xs_inter[i];
            let x_end = xs_inter[i + 1];

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