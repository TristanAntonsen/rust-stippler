mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
mod relax;
use canvas::{random_color, Canvas, Weighted_Canvas, random_grayscale};
use export::save_png;
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

const WIDTH: i32 =512;
const HEIGHT: i32 = 512;

fn main() {
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const _BLACK: [f32; 3] = [0.0, 0.0, 0.0];

    //main canvas
    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);
    let mut canvas2 = Canvas::new(WIDTH as usize, HEIGHT as usize);
    let mut canvas3 = Canvas::new(WIDTH as usize, HEIGHT as usize);

    //weight canvas
    let mut weight_canvas = Weighted_Canvas::from_image("blob.png");
    //creating start seeds
    // let mut seeds = Seeds::uniform(&canvas, 1000);
    let mut seeds = Seeds::rejection_sample(&weight_canvas, 150, 0.45);

    let start_seeds = seeds.clone();

    let relaxed = lloyd_relax(&start_seeds, 30, WIDTH as f64, "blob.png");

    for seed in start_seeds.coords {
        rasterize_circle(&seed, 3, _WHITE, &mut canvas2)
    }
    for seed in &relaxed {
        rasterize_circle(&seed, 3, _WHITE, &mut canvas3)
    }

    let vor_diagram = voronoi(relaxed, 1080.);

    let faces = voronoi::make_polygons(&vor_diagram);
    let mut sorted;
    let mut color;

    for face in faces {
        sorted = Unordered_Polygon::from_face(&face).sort();
        color = random_color();

        scanline_rasterize_polygon(&sorted, color, &mut canvas);
    }


    save_png("start_seeds.png", canvas2);
    save_png("end_seeds.png", canvas3);
    save_png("canvas.png", canvas);
}


