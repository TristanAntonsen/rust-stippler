use crate::geometry::{nearest_pixel, pixel, point, Line};
pub fn line_raster_bbox(line: &Line) -> [pixel; 2] {
    let x_min = f64::min(line.points[0].0, line.points[1].0);
    let x_max = f64::max(line.points[0].0, line.points[1].0);
    let y_min = f64::min(line.points[0].1, line.points[1].1);
    let y_max = f64::max(line.points[0].1, line.points[1].1);
    
    // min and max corners of bbox as pixels
    [
        nearest_pixel(&(x_min, x_max)),
        nearest_pixel(&(y_min, y_max)),
    ]
}
