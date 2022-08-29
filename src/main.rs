mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
use canvas::{random_color, Canvas};
use export::save_png;
use geometry::{nearest_pixel, vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use rasterize::{
    line_raster_bbox, polygon_raster_bbox, rasterize_circle, rasterize_line_naive,
    rasterize_polygon_boundary, scanline_nodes,
};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};

use crate::rasterize::scanline_rasterize_polygon;

fn main() {
    const WIDTH: i32 = 240;
    const HEIGHT: i32 = 240;
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);

    let mut seeds = Seeds::uniform(&canvas, 20);

    let vor_diagram = voronoi(seeds.coords, WIDTH as f64);

    let faces = voronoi::make_polygons(&vor_diagram);
    for face in &faces {
        let mut test_poly = Unordered_Polygon::from_face(face);
        let sorted_poly = test_poly.sort();

        let _c = vertex_centroid(&sorted_poly.vertices);
        let c = Point::new(_c[0], _c[1]);
        let color = random_color();
        scanline_rasterize_polygon(&sorted_poly, color, &mut canvas);
        // rasterize_circle(&c, 5, _RED, &mut canvas);
    }
    save_png("canvas.png", canvas);
}
