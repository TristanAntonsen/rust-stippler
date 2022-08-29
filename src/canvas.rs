use rand::Rng;
pub type color = [f32; 3];
pub struct Canvas {
    pub pixels: Vec<Vec<color>>,
}
pub struct Weighted_Canvas {
    pub pixel_weights: Vec<f32>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![vec![[0.0, 0.0, 0.0]; height]; width],
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: color) {
        if x < self.pixels[0].len() && y < self.pixels.len(){
            self.pixels[x][y] = color
        }
    }

    pub fn read_pixel(&self, x: usize, y: usize) -> color {
        if x < self.pixels[0].len() && y < self.pixels.len(){
            return self.pixels[x][y]
        } else {
            return [0.0,0.0,0.0] //return black
        }
    }

}

pub fn random_color() -> color {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen();
    let g: f32 = rng.gen();
    let b: f32 = rng.gen();

    [r,g,b]
}

pub fn random_grayscale() -> color {
    let mut rng = rand::thread_rng();
    let g: f32 = rng.gen();

    [g,g,g]
}