mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
use canvas::Canvas;
use export::save_png;
use geometry::{nearest_pixel, Line, Ordered_Polygon};
use rasterize::{line_raster_bbox, rasterize_line_naive, rasterize_polygon_boundary};
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

    let poly = Ordered_Polygon {
        vertices: vec![
            [40.0, 10.0],
            [180.0, 20.0],
            [210.0, 210.0],
            [90.0, 220.0],
            [10.0, 120.0]
        ]
    };

    rasterize_polygon_boundary(&poly, _GREEN, &mut canvas);

    // canvas.write_pixel(intersect_point[0] as usize, intersect_point[1] as usize, _RED);

    save_png("canvas.png", canvas);
}
