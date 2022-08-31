mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
mod relax;
use canvas::{random_color, Canvas, Weighted_Canvas, random_grayscale};
use export::save_png;
use geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use rasterize::{ rasterize_circle, scanline_rasterize_polygon, weighted_polygon_centroid};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};
use image::{ImageBuffer, Rgb, RgbImage, GenericImageView};
use image::io::Reader as ImageReader;

use crate::rasterize::raster_centroid;
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

    // //creating start seeds
    // let mut seeds = Seeds::uniform(&canvas, 20);
    // let start_seeds = seeds.clone();

    // //create voronoi diagram
    // let vor_diagram = voronoi(seeds.coords, WIDTH as f64);

    // //faces of diagram
    // let faces = voronoi::make_polygons(&vor_diagram);
    
    // //initializing variables
    // let (mut poly, mut sorted_poly);
    // let (mut _c, mut cV, mut cR, mut color);

    //creating weight array (grayscale)
    let mut weights = Weighted_Canvas::from_image("white.png");

    // //looping through voronoi regions
    // for face in &faces {
    //     //creating unordered polygon from region
    //     poly = Unordered_Polygon::from_face(face);
    //     // sorting ordered polygon
    //     sorted_poly = poly.sort();

    //     //creating the vertex centroid of the polygon
    //     _c = vertex_centroid(&sorted_poly.vertices);
    //     cV = Point::new(_c[0], _c[1]);
    //     cR = weighted_polygon_centroid(&sorted_poly, &mut weights);

    //     //visualizing polygon region
    //     color = random_color();
    //     scanline_rasterize_polygon(&sorted_poly,color, &mut canvas);

    //     //visualizing polygon VERTEX CENTROID in RED
    //     rasterize_circle(&cV, 4, _RED, &mut canvas);

    //     //visualizing polygon RASTER CENTROID in BLUE
    //     rasterize_circle(&cR, 4, _BLUE, &mut canvas);
    // }
    // //visualizing START SEEDS in GREEN
    // for point in &start_seeds.coords {
    //     rasterize_circle(&point, 4, _GREEN, &mut canvas)
    // }

    let hex = Ordered_Polygon::ngon([206., 300.], 200.0, 48);
    scanline_rasterize_polygon(&hex, _GREEN, &mut canvas);

    let cV = vertex_centroid(&hex.vertices);
    let cR = raster_centroid(&hex, &mut canvas);

    //visualizing polygon VERTEX CENTROID in RED
    rasterize_circle(&Point::new(cV[0], cV[1]), 4, _RED, &mut canvas);

    //visualizing polygon RASTER CENTROID in BLUE
    rasterize_circle(&cR, 4, _BLUE, &mut canvas);

    save_png("canvas.png", canvas);
}


