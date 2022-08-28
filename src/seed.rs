use crate::Canvas;
use rand::Rng;


pub struct Seeds {
    pub coords : Vec<(f64, f64)>
}

impl Seeds {
    pub fn uniform(canvas: &Canvas, count: usize) -> Self {
        let width = canvas.pixels[0].len();
        let height = canvas.pixels.len();
        let mut seeds = Vec::new();
        let mut rng = rand::thread_rng();
        let mut x: f64;
        let mut y: f64;
        let mut n = 0;
        while n < count {
            x = rng.gen_range(0..width) as f64;
            y = rng.gen_range(0..height) as f64;
            seeds.push((x,y));
            n += 1
        }
        Self {
            coords: seeds
        }
    }

}