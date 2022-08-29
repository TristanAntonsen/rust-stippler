use rand::Rng;

use crate::geometry::pixel;
pub type color = [f32; 3];
pub struct Canvas {
    pub pixels: Vec<Vec<color>>,
}
pub struct Weighted_Canvas { //grayscale
    pub pixel_weights: Vec<Vec<f32>>,
}

impl Weighted_Canvas {
    pub fn read_pixel(&self, x: usize, y: usize) -> f32 {
        if x < self.pixel_weights[0].len() && y < self.pixel_weights.len() {
            return self.pixel_weights[x][y];
        } else {
            return 0.0; //return black
        }
    }
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![[0.0, 0.0, 0.0]; height]; width],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: color) {
        if x < self.pixels[0].len() && y < self.pixels.len() {
            self.pixels[x][y] = color
        }
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> color {
        if x < self.pixels[0].len() && y < self.pixels.len() {
            return self.pixels[x][y];
        } else {
            return [0.0, 0.0, 0.0]; //return black
        }
    }
    pub fn to_grayscale(&mut self) -> Weighted_Canvas {
        let w = self.pixels[0].len();
        let h = self.pixels.len();

        let mut weights = vec![vec![0.0; w]; h];

        for x in 0..w {
            for y in 0..h {
                weights[y][x] = color_average(&self.pixels[x][y]);
            }
        }

        Weighted_Canvas {
            pixel_weights: weights,
        }
    }
}

pub fn random_color() -> color {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen();
    let g: f32 = rng.gen();
    let b: f32 = rng.gen();

    [r, g, b]
}

pub fn random_grayscale() -> color {
    let mut rng = rand::thread_rng();
    let g: f32 = rng.gen();

    [g, g, g]
}

pub fn color_average(color: &color) -> f32 {

    (color[0] + color[1] + color[2]) as f32 / 3.0

}