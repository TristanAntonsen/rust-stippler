mod export;
mod canvas;
mod geometry;
mod seed;
mod rasterize;
use canvas::Canvas;
use export::save_png;
use geometry::{nearest_pixel, Line};
use rasterize::{line_raster_bbox, rasterize_line_naive};
use seed::Seeds;
extern crate voronoi;
use voronoi::{voronoi, Point, make_polygons, make_line_segments};

fn main() {
    const WIDTH: i32 = 500;
    const HEIGHT: i32 = 500;
    const RED: [f32; 3] = [1.0, 0.0, 0.0];
    const GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const WHITE: [f32; 3] = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);

    let seeds = Seeds::uniform(&canvas, 10);
    let mut pixel;
    for s in &seeds.coords {
        pixel = nearest_pixel(s);
        canvas.write_pixel(pixel[0] as usize, pixel[1] as usize, WHITE);
        // println!("{:?}",s)
    }
    let box_size: f64 = 500.0;
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
        println!("Line segment: {:?}",seg);
        rasterize_line_naive(&line, RED, &mut canvas);
        let p1 = nearest_pixel(&seg[0]);
        let p2 = nearest_pixel(&seg[1]);
        for p in [p1, p2] {
            if p[0] < WIDTH && p[0]>= 0 && p[1] < HEIGHT && p[1] >= 0 {      
            canvas.write_pixel(p1[0] as usize, p1[1] as usize, GREEN);
            }
        }
    }

    save_png("canvas.png", canvas);

}
