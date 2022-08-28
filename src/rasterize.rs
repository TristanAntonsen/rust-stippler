use crate::geometry::{nearest_pixel, pixel, point, Ordered_Polygon, Line};
use crate::Canvas;
use voronoi::Point;

pub const _TEST_LINE: Line =  Line {
    points: [
        [200.0, 10.0],
        [40.0, 200.0],
    ]
};


pub fn line_raster_bbox(line: &Line) -> [pixel; 2] {
    let x_min = f64::min(line.points[0][0], line.points[1][0]);
    let x_max = f64::max(line.points[0][0], line.points[1][0]);
    let y_min = f64::min(line.points[0][1], line.points[1][1]);
    let y_max = f64::max(line.points[0][1], line.points[1][1]);

    // min and max values of bbox as pixels
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

pub fn rasterize_polygon_boundary(poly: &Ordered_Polygon, color: [f32; 3], canvas: &mut Canvas) {

    let edges = poly.create_edges();
    for edge in edges {
        rasterize_line_naive(&edge, color, canvas)
    }
}

pub fn scanline_nodes(poly: &Ordered_Polygon, scan_y: f64, width: f64) -> Vec<pixel> {

    let mut nodes = Vec::new();
    let mut p1y;
    let mut p2y;
    let mut p1x;
    let mut p2x;
    let mut scanline;
    let mut line;
    let mut node: pixel;
    
    for edge in poly.create_edges() {

        p1y = edge.points[0][1];
        p2y = edge.points[1][1];

        // If line crosses scanline
        if p1y < scan_y && p2y >= scan_y || p1y >= scan_y && p2y < scan_y{
            p1x = edge.points[0][0];
            p2x = edge.points[1][0];

            if p1x < 0.0 {
                p1x = 0.0
            }
            scanline = Line {
                points: [
                    [p1x,p1y],
                    [p2x,p2y]
                ]
            };
            line = Line {
                points: [
                    [0.0,scan_y],
                    [width, scan_y]
                ]
            };

            node = scanline.line_intersection(&line);
            node[0] += 1; // accounts for line shifting left
            nodes.push(node)
        }
    }

    nodes

    // nodes
}

// def Scanline_nodes(polygon,scan_y, image_res):

//     polygon_edges = Edges(polygon)
//     nodes = []
//     for edge in polygon_edges:
//         edge = list(edge)
//         p1y = edge[0][1]
//         p2y = edge[1][1]

//         ## If line crosses scanline
//         if p1y < scan_y and p2y >= scan_y or p1y >= scan_y and p2y < scan_y:
//             p1x = edge[0][0]
//             p2x = edge[1][0]

//             if p1x < 0:
//                 p1x = 0
//             if p1x >= image_res:
//                 p1x = image_res - 1

//             p1 = [p1x,p1y]
//             p2 = [p2x,p2y]
//             p3 = [0,scan_y]
//             p4 = [image_res,scan_y]
//             node = Line_Intersection(p1,p2,p3,p4)
//             node = [Clamp(node[0],[0,image_res]),Clamp(node[1],[0,image_res])]
//             nodes.append(node)

//     if len(nodes) < 1:
//         return False
//     else:
//         return nodes