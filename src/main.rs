mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
mod relax;
use canvas::{random_color, Canvas, Weighted_Canvas, random_grayscale};
use export::save_png;
use geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use rasterize::{ rasterize_circle, scanline_rasterize_polygon, weighted_raster_centroid};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};
use image::{ImageBuffer, Rgb, RgbImage, GenericImageView};
use image::io::Reader as ImageReader;

use crate::rasterize::raster_centroid;
use crate::relax::lloyd_relax;
// use relax::lloyd_relax;

const WIDTH: i32 = 512;
const HEIGHT: i32 = 512;

fn main() {
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];
    const _BLACK: [f32; 3] = [0.0, 0.0, 0.0];

    //main canvas
    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);

    //creating start seeds
    let mut seeds = Seeds::uniform(&canvas, 20);
    let start_seeds = seeds.clone();

    //create voronoi diagram
    let vor_diagram = voronoi(seeds.coords, WIDTH as f64);

    //faces of diagram
    let faces = voronoi::make_polygons(&vor_diagram);
    
    //initializing variables
    let (mut poly, mut sorted_poly);
    let (mut _c, mut cV, mut cR, mut cW, mut color);

    //creating weight array (grayscale)
    let mut weights = Weighted_Canvas::from_image("gradient.png");

    //looping through voronoi regions
    for face in &faces {
        //creating unordered polygon from region
        poly = Unordered_Polygon::from_face(face);
        // sorting ordered polygon
        sorted_poly = poly.sort();

        //creating the vertex centroid of the polygon
        _c = vertex_centroid(&sorted_poly.vertices);
        cV = Point::new(_c[0], _c[1]);
        cR = weighted_raster_centroid(&sorted_poly, &mut weights);
        cW = raster_centroid(&sorted_poly, &mut canvas);

        //visualizing polygon region
        color = random_color();
        // scanline_rasterize_polygon(&sorted_poly,color, &mut canvas);

        // //visualizing polygon WEIGHTED RASTER CENTROID in BLUE
        // rasterize_circle(&cW, 4, _WHITE, &mut canvas);
    }
    //visualizing START SEEDS in GREEN
    for point in &start_seeds.coords {
        rasterize_circle(&point, 4, _GREEN, &mut canvas)
    }

    let relaxed = lloyd_relax(&start_seeds, 2, WIDTH as f64, "canvas.png");
    //visualizing RELAXED SEEDS in BLUE
    for point in &relaxed {
        rasterize_circle(&point, 4, _BLUE, &mut canvas)
    }
    save_png("canvas.png", canvas);
}


