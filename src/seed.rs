use crate::Canvas;
use rand::Rng;


pub struct Seeds {
    pub coords : Vec<(f32, f32)>
}

impl Seeds {
    pub fn uniform(canvas: &Canvas, count: usize) -> Self {
        let width = canvas.pixels[0].len();
        let height = canvas.pixels.len();
        let mut seeds = Vec::new();
        let mut rng = rand::thread_rng();
        let mut x: f32;
        let mut y: f32;
        let mut n = 0;
        while n < count {
            x = rng.gen_range(0..width) as f32;
            y = rng.gen_range(0..height) as f32;
            seeds.push((x,y));
            n += 1
        }
        Self {
            coords: seeds
        }
    }

}