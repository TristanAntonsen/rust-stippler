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
    // let file_path = "sampling/circle512.png";
    let file_path = &image_path;
    let mut weight_canvas = Weighted_Canvas::from_image(file_path);
    let WIDTH = weight_canvas.pixel_weights[0].len();
    let HEIGHT = weight_canvas.pixel_weights.len();
    
    //main canvas1
    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);
    let mut canvas2 = Canvas::solid_color(WIDTH as usize, HEIGHT as usize, _WHITE);
    let mut canvas3 = Canvas::solid_color(WIDTH as usize, HEIGHT as usize, _WHITE);


    //creating start seeds
    // let mut seeds = Seeds::uniform(&canvas, points);
    let mut seeds = Seeds::rejection_sample(&weight_canvas,points, threshold);
    // let mut seeds = Seeds::cartesian(&weight_canvas,60.0);

    let start_seeds = seeds.clone();

    let relaxed = lloyd_relax(&start_seeds, iterations, WIDTH as f64, file_path);

    for seed in start_seeds.coords {
        rasterize_circle(&seed, 2, _BLACK, &mut canvas2)
    }
    for seed in &relaxed {
        rasterize_circle(&seed, 2, _BLACK, &mut canvas3)
    }

    let vor_diagram = voronoi(relaxed, WIDTH as f64);
    let mut color_canvas = Canvas::new(WIDTH, HEIGHT);

    let faces = voronoi::make_polygons(&vor_diagram);
    // let mut sorted;
    // let mut color;

    // for face in faces {
    //     sorted = Unordered_Polygon::from_face(&face).sort();
    //     color = random_color();
    //     // centroid = weighted_raster_centroid(&sorted, &mut weight_canvas);

    //     scanline_rasterize_polygon(&sorted, color, &mut color_canvas);
    // }
    color_sampled_voronoi(file_path, faces, &mut color_canvas, &mut weight_canvas);
    
    save_rgb_image("voronoi_colors.png", color_canvas);
    save_image("start_seeds.jpg", canvas2);
    save_image("end_seeds.jpg", canvas3);
}


