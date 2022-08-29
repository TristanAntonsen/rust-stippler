mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
use canvas::{random_color, Canvas, random_grayscale};
use export::save_png;
use geometry::{nearest_pixel, vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use rasterize::{
    line_raster_bbox, polygon_raster_bbox, rasterize_circle, rasterize_line_naive,
    rasterize_polygon_boundary, scanline_nodes, scanline_rasterize_polygon
};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};


fn main() {
    const WIDTH: i32 = 240;
    const HEIGHT: i32 = 240;
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);

    let mut seeds = Seeds::uniform(&canvas, 20);
    let start_seeds = seeds.clone();
    let vor_diagram = voronoi(seeds.coords, WIDTH as f64);

    let faces = voronoi::make_polygons(&vor_diagram);
    let mut poly;
    let mut sorted_poly;
    let mut _c;
    let mut c;
    let mut color;
    for face in &faces {
        poly = Unordered_Polygon::from_face(face);
        sorted_poly = poly.sort();

        _c = vertex_centroid(&sorted_poly.vertices);
        c = Point::new(_c[0], _c[1]);
        // color = random_color();
        color = random_grayscale();
        scanline_rasterize_polygon(&sorted_poly,color, &mut canvas);
        //Skips lines -- is it the floor() function?
        //somehow the ys are duplicating/lagging
        rasterize_circle(&c, 2, _BLUE, &mut canvas);
    }
    for point in &start_seeds.coords {
        rasterize_circle(point, 2, _RED, &mut canvas)
    }
    save_png("canvas.png", canvas);
}
