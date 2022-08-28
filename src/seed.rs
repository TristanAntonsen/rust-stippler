use crate::Canvas;
use rand::Rng;
// use rand::prelude::*;
use rand::distributions::Uniform;
use voronoi::Point;

pub struct Seeds {
    pub coords : Vec<Point>
}

impl Seeds {
    pub fn uniform(canvas: &Canvas, count: usize) -> Self {
        let width = canvas.pixels[0].len();
        let height = canvas.pixels.len();

        let mut rng = rand::thread_rng();
        let range1 = Uniform::new(0., width as f64);
        let range2 = Uniform::new(0., height as f64);
        let seeds: Vec<Point> = (0..count)
        .map(|_| Point::new(rng.sample(&range1), rng.sample(&range2)))
        .collect();
        Self {
            coords: seeds
        }
    }

}