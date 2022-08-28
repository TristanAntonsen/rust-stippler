pub type color = [f32; 3];
pub struct Canvas {
    pub pixels: Vec<Vec<color>>,
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
}