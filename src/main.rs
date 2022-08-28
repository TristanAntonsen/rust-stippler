mod canvas;
mod export;
mod geometry;
mod rasterize;
mod seed;
use canvas::Canvas;
use export::save_png;
use geometry::{nearest_pixel, Line};
use rasterize::{line_raster_bbox, rasterize_line_naive, _TEST_LINE};
use seed::Seeds;
extern crate voronoi;
use voronoi::{make_line_segments, make_polygons, voronoi, Point};

fn main() {
    const WIDTH: i32 = 240;
    const HEIGHT: i32 = 240;
    const _RED: [f32; 3] = [1.0, 0.0, 0.0];
    const _GREEN: [f32; 3] = [0.0, 1.0, 0.0];
    const _BLUE: [f32; 3] = [0.0, 0.0, 1.0];
    const _WHITE: [f32; 3] = [1.0, 1.0, 1.0];

    let mut canvas = Canvas::new(WIDTH as usize, HEIGHT as usize);
    let line1 =  Line {
        points: [
            [200.0, 10.0],
            [40.0, 200.0],
        ]
    };
    let line2 =  Line {
        points: [
            [10.0, 10.0],
            [200.0, 200.0],
        ]
    };
    let intersect_point = line1.line_intersection(&line2);
    rasterize_line_naive(&line1, _BLUE, &mut canvas);
    rasterize_line_naive(&line2, _GREEN, &mut canvas);
    canvas.write_pixel(intersect_point[0] as usize, intersect_point[1] as usize, _RED);

    save_png("canvas.png", canvas);
}
