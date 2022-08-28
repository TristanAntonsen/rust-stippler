mod export;
mod canvas;
mod geometry;
mod seed;
mod rasterize;
use canvas::Canvas;
use export::save_png;
use geometry::{nearest_pixel, Line};
use rasterize::{line_raster_bbox, rasterize_line_naive};
use seed::Seeds;
extern crate voronoi;
use voronoi::{voronoi, Point, make_polygons};

fn main() {
    const WIDTH: usize = 500;
    const HEIGHT: usize = 500;
    const RED: [f32; 3] = [1.0, 0.0, 0.0];
    const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let seeds = Seeds::uniform(&canvas, 100);
    let mut pixel;
    for s in &seeds.coords {
        pixel = nearest_pixel(s);
        canvas.write_pixel(pixel[0], pixel[1], WHITE);
        // println!("{:?}",s)
    }
    let box_size: f64 = 500.0;
    let vor_diagram = voronoi(seeds.coords, box_size);

    save_png("canvas.png", canvas);

}
