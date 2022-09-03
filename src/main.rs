mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
mod relax;
use std::ops::Mul;

use canvas::{random_color, Canvas, Weighted_Canvas, random_grayscale};
use export::save_image;
use geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use rasterize::{ rasterize_circle, scanline_rasterize_polygon,
    weighted_raster_centroid, test_centroid};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};
use image::{ImageBuffer, Rgb, RgbImage, GenericImageView};
use image::io::Reader as ImageReader;

use crate::rasterize::{raster_centroid, rasterize_line_naive};
use crate::relax::lloyd_relax;
// use relax::lloyd_relax;

// const WIDTH: i32 =512;
// const HEIGHT: i32 = 512;

fn main() {
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const _BLACK: [f32; 3] = [0.0, 0.0, 0.0];

    //weight canvas
    let file_path = "sampling/mannequin.jpg";
    let mut weight_canvas = Weighted_Canvas::from_image(file_path);
    let WIDTH = weight_canvas.pixel_weights[0].len();
    let HEIGHT = weight_canvas.pixel_weights.len();
    
    //main canvas1
    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);
    let mut canvas2 = Canvas::new(WIDTH as usize, HEIGHT as usize);
    let mut canvas3 = Canvas::solid_color(WIDTH as usize, HEIGHT as usize, _WHITE);


    //creating start seeds
    // let mut seeds = Seeds::uniform(&canvas, 1000);
    let mut seeds = Seeds::rejection_sample(&weight_canvas, 1000, 1.0);

    let start_seeds = seeds.clone();

    let relaxed = lloyd_relax(&start_seeds, 90, WIDTH as f64, file_path);

    for seed in start_seeds.coords {
        rasterize_circle(&seed, 3, _BLACK, &mut canvas2)
    }
    for seed in &relaxed {
        rasterize_circle(&seed, 3, _BLACK, &mut canvas3)
    }
    // let mut scaled;
    // for seed in &relaxed {
    //     scaled = Point::new(
    //         f64::try_from(seed.x).unwrap() * 2.0,
    //         f64::try_from(seed.y).unwrap() * 2.0
    //     );
    //     rasterize_circle(&scaled, 1, _WHITE, &mut canvas3)
    // }

    let vor_diagram = voronoi(relaxed, 1080.);

    let faces = voronoi::make_polygons(&vor_diagram);
    let mut sorted;
    let mut color;

    for face in faces {
        sorted = Unordered_Polygon::from_face(&face).sort();
        color = random_color();

        scanline_rasterize_polygon(&sorted, color, &mut canvas);
    }


    save_image("start_seeds.jpg", canvas2);
    save_image("end_seeds.jpg", canvas3);
    save_image("canvas.jpg", canvas);
}


