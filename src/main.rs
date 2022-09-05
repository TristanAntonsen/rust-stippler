mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
mod relax;
mod cli;

use std::env;
use canvas::{random_color, Canvas, Weighted_Canvas};
use export::{save_image, save_rgb_image};
use geometry::Unordered_Polygon;
use rasterize::{rasterize_circle, scanline_rasterize_polygon,weighted_raster_centroid, color_sampled_voronoi};
use seed::Seeds;
extern crate voronoi;
use voronoi::voronoi;
use relax::lloyd_relax;
use structopt::StructOpt;
use cli::Opt;

fn main() {
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const _BLACK: [f32; 3] = [0.0, 0.0, 0.0];

    // ARGUMENTS //

    let opt = Opt::from_args();
    let image_path = opt.input.to_str().unwrap();
    let points: usize = opt.count;
    let iterations: u16 = opt.iterations;
    let threshold: f32 = opt.threshold;
    let cartesian_spacing: u32 = opt.cartesian_spacing;
    let save_frames: bool = opt.frames;
    

    // --------- //
    //weight canvas
    let file_path = &image_path;
    let mut weight_canvas = Weighted_Canvas::from_image(file_path);
    let WIDTH = weight_canvas.pixel_weights[0].len();
    let HEIGHT = weight_canvas.pixel_weights.len();
    
    //main canvas1
    let mut canvas2 = Canvas::solid_color(WIDTH as usize, HEIGHT as usize, _WHITE);
    let mut canvas3 = Canvas::solid_color(WIDTH as usize, HEIGHT as usize, _WHITE);
    let mut color_canvas = Canvas::new(WIDTH, HEIGHT);


    //creating start seeds
    // let mut seeds = Seeds::uniform(&canvas2, points);
    // let mut seeds = Seeds::cartesian(&weight_canvas,cartesian_spacing as f64, threshold);
    let seeds = Seeds::rejection_sample(&weight_canvas,points, threshold);
    // let seeds = Seeds::pdf_rejection_sample(&weight_canvas,points, threshold);

    let relaxed = lloyd_relax(&seeds, iterations, WIDTH as f64, file_path, save_frames);

    for seed in seeds.coords {
        rasterize_circle(&seed, 2, _BLACK, &mut canvas2)
    }
    for seed in &relaxed {
        rasterize_circle(&seed, 2, _BLACK, &mut canvas3)
    }

    let vor_diagram = voronoi(relaxed, WIDTH as f64);

    let faces = voronoi::make_polygons(&vor_diagram);

    color_sampled_voronoi(file_path, faces, &mut color_canvas, &mut weight_canvas);
    
    // save_image("sequence/0.jpg", canvas2);
    save_image("start_seeds.jpg", canvas2);
    save_image("end_seeds.jpg", canvas3);
    save_rgb_image("voronoi_colors.png", color_canvas);
}


