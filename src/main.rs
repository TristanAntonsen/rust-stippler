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
extern crate voronator;
use voronator::VoronoiDiagram;
use voronator::delaunator::Point;

fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 120;
    const RED: [f32; 3] = [1.0, 0.0, 0.0];
    const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let seeds = Seeds::uniform(&canvas, 100);
    let mut pixel;
    for s in seeds.coords {
        pixel = nearest_pixel(&s);
        canvas.write_pixel(pixel[0], pixel[1], WHITE);
        // println!("{:?}",s)
    }
    let line = Line {
        points: [(10.0, 10.0),(100.0, 100.0)]
    };

    rasterize_line_naive(&line, RED, &mut canvas);

    save_png("canvas.png", canvas);

}
