mod export;
mod canvas;
mod geometry;
mod seed;
use canvas::Canvas;
use export::save_png;
use geometry::{nearest_pixel};
use seed::Seeds;
extern crate voronator;
use voronator::VoronoiDiagram;
use voronator::delaunator::Point;

fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 120;
    let WHITE = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let seeds = Seeds::uniform(&canvas, 100);
    let mut pixel;
    for s in seeds.coords {
        pixel = nearest_pixel(&s);
        canvas.write_pixel(pixel[0], pixel[1], WHITE);
        // println!("{:?}",s)
    }
    
    save_png("canvas.png", canvas);

}
