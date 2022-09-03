use crate::{Canvas, Weighted_Canvas, canvas::color};
extern crate image;
use image::{Rgb, RgbImage};
use voronoi::Point;
use crate::seed::Seeds;
use crate::rasterize_circle;

pub fn save_image(path: &str, canvas: Canvas) {
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
    
    img.save(path).expect("Could not save image");
}


pub fn save_grayscale_png(path: &str, canvas: Weighted_Canvas) { // rework with generics/traits later
    let width = canvas.pixel_weights.len() as u32;
    let height = canvas.pixel_weights[0].len() as u32;

    let mut img = RgbImage::new(width, height);
    let mut g;
    let mut value: f32;
    for x in 0..width {
        for y in 0..height {
            value = canvas.pixel_weights[x as usize][y as usize];
            g = (value * 255.0).round() as u8;
            
            img.put_pixel(x, y, Rgb([g,g,g])); //just using RGB with equal r/g/b values        
        }
    }
    println!("{} exported.", path);
    
    img.save(path).expect("Could not save png");
}

pub fn visualize_frame(frame: u16, seeds: &Seeds, width: usize, height: usize, scale: usize, background_color: color, dot_color: color ) {
    let mut canvas = Canvas::solid_color(width * scale, width * scale,background_color);
    let mut file_name = "sequence/".to_string();
    let mut scaled_point: Point;
    let (mut x, mut y);
    for point in &seeds.coords {
        //hacky, need to fix this
        x = f64::try_from(point.x).unwrap() * scale as f64;
        y = f64::try_from(point.y).unwrap() * scale as f64;
        scaled_point = Point::new(x,y);
        rasterize_circle(&scaled_point, 4, [0.0, 0.0, 0.0], &mut canvas)
    }
    
    file_name.push_str(&frame.to_string());
    file_name.push_str(".jpg");
    save_image(&file_name[..], canvas);
}