use crate::geometry::{nearest_pixel, pixel, point, Line};
use crate::Canvas;
use voronoi::Point;

pub fn line_raster_bbox(line: &Line) -> [pixel; 2] {
    let x_min = f64::min(line.points[0][0], line.points[1][0]);
    let x_max = f64::max(line.points[0][0], line.points[1][0]);
    let y_min = f64::min(line.points[0][1], line.points[1][1]);
    let y_max = f64::max(line.points[0][1], line.points[1][1]);

    // min and max corners of bbox as pixels
    [
        nearest_pixel(&Point::new(x_min, x_max)),
        nearest_pixel(&Point::new(y_min, y_max)),
    ]
}
pub fn rasterize_line_naive(line: &Line, color: [f32; 3], canvas: &mut Canvas) {
    let line_bbox = line_raster_bbox(&line);

    let x1 = line.points[0][0].floor() as i32;
    let y1 = line.points[0][1].floor() as i32;
    let x2 = line.points[1][0].floor() as i32;
    let y2 = line.points[1][1].floor() as i32;

    let dx = x2 - x1;
    let dy = y2 - y1;
    
    let mut y;
    for x in i32::min(x1, x2)..i32::max(x1, x2) {
        y = y1 + dy * (x - x1) / dx;
        canvas.write_pixel(x as usize, y as usize, color);
    }
}
