use std::fs;
use crate::Canvas;
extern crate image;
use image::{ImageBuffer, Rgb, RgbImage};


pub fn save_png(path: &str, canvas: Canvas) {
    let width = canvas.pixels.len() as u32;
    let height = canvas.pixels[0].len() as u32;

    let mut img = RgbImage::new(width, height);
    let mut r;
    let mut g;
    let mut b;
    let mut color: [f32; 3];
    for x in 0..width {
        for y in 0..height {
            color = canvas.pixels[x as usize][y as usize];
            r = (color[0] * 255.0).round() as u8;
            g = (color[1] * 255.0).round() as u8;
            b = (color[2] * 255.0).round() as u8;
            
            img.put_pixel(x, y, Rgb([r,g,b]));            
        }
    }
    println!("{} exported.", path);
    
    img.save(path).expect("Could not save png");
}
