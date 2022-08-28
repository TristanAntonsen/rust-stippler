mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
use canvas::Canvas;
use export::save_png;
use geometry::{nearest_pixel, Line, Ordered_Polygon};
use rasterize::{
    line_raster_bbox, polygon_raster_bbox, rasterize_line_naive, rasterize_polygon_boundary,
    scanline_nodes,
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

    let poly = Ordered_Polygon {
        vertices: vec![
            [40.0, 10.0],
            [180.0, 20.0],
            [210.0, 210.0],
            [90.0, 220.0],
            [10.0, 120.0],
        ],
    };
    let bbox = polygon_raster_bbox(&poly);
    let mut nodes;
    let mut scanline;
    for y in bbox[1][0]..bbox[1][1] {
        //for y in y_min to y_max of polygon bbox
        println!("{}", y);
        nodes = scanline_nodes(&poly, y as f64, WIDTH as f64);
        if nodes.len() > 0 {
            scanline = Line::from_nodes(&nodes);
            rasterize_line_naive(&scanline, _WHITE, &mut canvas);
            canvas.write_pixel(nodes[0][0] as usize, nodes[0][1] as usize, _RED);
            canvas.write_pixel(nodes[1][0] as usize, nodes[1][1] as usize, _GREEN);
        }
    }

    save_png("canvas.png", canvas);
}
