mod export;
mod canvas;
mod geometry;
use canvas::Canvas;
use export::save_png;
use geometry::Ordered_Polygon;
fn main() {

    let mut canvas = Canvas::new(120, 120);
    let scale = 10.0;
    let polygon = Ordered_Polygon {
        vertices: vec!(
            [2.5 * scale, 0.0 * scale],
            [5.0 * scale, 2.5 * scale],
            [2.5 * scale, 5.0 * scale],
            [0.0 * scale, 2.5 * scale],
            )
    };
    for vertex in polygon.vertices {
        canvas.write_pixel(vertex[0].round() as usize, vertex[1].round() as usize, [1.0, 1.0, 1.0])
    }
    save_png("canvas.png", canvas);

}
