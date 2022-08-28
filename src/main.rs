mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
use canvas::Canvas;
use export::save_png;
use geometry::{nearest_pixel, Line};
use rasterize::{line_raster_bbox, rasterize_line_naive};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};

fn main() {
    const WIDTH: i32 = 1080;
    const HEIGHT: i32 = 1080;
    const RED: [f32; 3] = [1.0, 0.0, 0.0];
    const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);

    let seeds = Seeds::uniform(&canvas, 200);
    let mut pixel;
    for s in &seeds.coords {
        pixel = nearest_pixel(s);
        canvas.write_pixel(pixel[0] as usize, pixel[1] as usize, WHITE);
        // println!("{:?}",s)
    }
    let box_size = WIDTH as f64;
    let vor_diagram = voronoi(seeds.coords, box_size);
    let vor_segments = make_line_segments(&vor_diagram);

    // let line = Line {
    //     points: [
    //         [300.0, 100.0],
    //         [100.0, 200.0],
    //         // [100.0, 100.0],
    //         // [300.0, 200.0],
    //     ]
    // };
    // rasterize_line_naive(&line, WHITE, &mut canvas);
    let mut line;
    for seg in &vor_segments {
        line = Line::from_segment(*seg);
        let p1 = nearest_pixel(&seg[0]);
        let p2 = nearest_pixel(&seg[1]);
        for p in [p1, p2] {
            rasterize_line_naive(&line, RED, &mut canvas);
        }
    }

    save_png("canvas.png", canvas);
}
