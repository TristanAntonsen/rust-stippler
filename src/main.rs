mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
mod relax;
use std::env;

use canvas::{random_color, Canvas, Weighted_Canvas};
use export::{save_image, save_rgb_image};
use geometry::Unordered_Polygon;
use rasterize::{rasterize_circle, scanline_rasterize_polygon,weighted_raster_centroid, color_sampled_voronoi};
use seed::Seeds;
extern crate voronoi;
use voronoi::voronoi;
use relax::lloyd_relax;


fn main() {
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const _BLACK: [f32; 3] = [0.0, 0.0, 0.0];

    // ARGUMENTS //
    let args: Vec<String> = env::args().collect();
    let image_path = &args[1];
    let points: usize = args[2].to_string().parse().unwrap();
    let iterations: u16 = args[3].to_string().parse().unwrap();
    let threshold: f32 = args[4].to_string().parse().unwrap();
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
    // let mut seeds = Seeds::uniform(&canvas, points);
    // let mut seeds = Seeds::cartesian(&weight_canvas,60.0);
    let seeds = Seeds::rejection_sample(&weight_canvas,points, threshold);

    let relaxed = lloyd_relax(&seeds, iterations, WIDTH as f64, file_path);

    for seed in seeds.coords {
        rasterize_circle(&seed, 2, _BLACK, &mut canvas2)
    }
    for seed in &relaxed {
        rasterize_circle(&seed, 2, _BLACK, &mut canvas3)
    }

    let vor_diagram = voronoi(relaxed, WIDTH as f64);

    let faces = voronoi::make_polygons(&vor_diagram);

    color_sampled_voronoi(file_path, faces, &mut color_canvas, &mut weight_canvas);
    
    save_image("seeds.jpg", canvas2);
    save_image("end_seeds.jpg", canvas3);
    save_rgb_image("voronoi_colors.png", color_canvas);
}


