mod export;
mod canvas;
mod geometry;
mod seed;
use canvas::Canvas;
use export::save_png;
use geometry::{Ordered_Polygon, point, Line};
use seed::Seeds;
extern crate voronator;
use voronator::VoronoiDiagram;
use voronator::delaunator::Point;

fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 120;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    let seeds = Seeds::uniform(&canvas, 100);
    
    for s in seeds.coords {
        println!("{:?}",s)
    }
}
