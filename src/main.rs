mod export;
mod canvas;
mod geometry;
mod seed;
use canvas::Canvas;
use export::save_png;
use geometry::{Ordered_Polygon, point, Line};
use seed::Seeds;
use rand::Rng;
fn main() {
    const WIDTH: usize = 120;
    const HEIGHT: usize = 120;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let scale = 10.0;
    let polygon = Ordered_Polygon {
        vertices: vec!(
            [2.5 * scale, 0.0 * scale],
            [5.0 * scale, 2.5 * scale],
            [2.5 * scale, 5.0 * scale],
            [0.0 * scale, 2.5 * scale],
            )
    };


    let point: point = [0.0,5.0];
    let seeds = Seeds::uniform(&canvas, 100);

    println!("Seeds: {:?}", seeds.coords);    

    for seed in seeds.coords {
        canvas.write_pixel(seed[0].round() as usize, seed[1].round() as usize, [1.0, 1.0, 1.0])
    }
    save_png("canvas.png", canvas);
}
