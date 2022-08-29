use crate::geometry::{distance, nearest_pixel, pixel, point, Line, Ordered_Polygon};
use crate::{Canvas, Weighted_Canvas};
use voronoi::Point;

pub const _TEST_LINE: Line = Line {
    points: [[200.0, 10.0], [40.0, 200.0]],
};


pub fn weighted_polygon_centroid(poly: &Ordered_Polygon, weights: &mut Weighted_Canvas) -> Option<Point>{
    let width = weights.pixel_weights[0].len() as f64;
    let bbox = polygon_raster_bbox(&poly);
    let mut nodes;
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut value;
    let mut weight;
    let mut total_weight = 0.0;
    let mut pixel_count = 0;
    for y in bbox[1][0]..bbox[1][1] {
        nodes = scanline_nodes(&poly, y as f64, width);
        if nodes.len() > 0 {
            for x in nodes[0][0]..nodes[1][0] {
                value = weights.read_pixel(x as usize, y as usize);
                weight = 1.0 - value;
                total_weight += weight;

                cx += x as f32 * weight;
                cy += y as f32 * weight;
                
                pixel_count += 1;

            }
            // calculate weights for pixels between nodes (including ends?)
        }
    }

    if total_weight == 0.0 {
        total_weight = pixel_count as f32;
    }

    cx /= total_weight;
    cy /= total_weight;

    if pixel_count == 0 {
        return None
    };
    let centroid = Point::new(cx as f64, cy as f64);
    
    Some(centroid)

}

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
pub fn polygon_raster_bbox(poly: &Ordered_Polygon) -> [[i32; 2]; 2] {
    let edges = poly.create_edges();
    // let mut x_min = edges[0].points[0][0]; //arbitrarily choosing first edge point
    // let mut y_min = edges[0].points[1][0];
    // let mut x_max = edges[0].points[0][0];
    // let mut y_max = edges[0].points[1][0];
    let mut x_vals = Vec::new();
    let mut y_vals = Vec::new();
    for e in edges {
        x_vals.push(e.points[0][0]);
        x_vals.push(e.points[1][0]);
        y_vals.push(e.points[0][1]);
        y_vals.push(e.points[1][1]);
    }
    let n = x_vals.len();
    x_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    y_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let x_min = x_vals[0];
    let y_min = y_vals[0];
    let x_max = x_vals[n - 1];
    let y_max = y_vals[n - 1];

    // min and max values of bbox as pixels
    [
        nearest_pixel(&Point::new(x_min, x_max)), // (min_x, max_x)
        nearest_pixel(&Point::new(y_min, y_max)), // (min_y, max_y)
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

pub fn scanline_rasterize_polygon(poly: &Ordered_Polygon, color: [f32; 3], canvas: &mut Canvas) {
    let width = canvas.pixels[0].len() as f64;
    let bbox = polygon_raster_bbox(&poly);
    let mut nodes;
    let mut scanline;
    // println!("bbox: {},{}",bbox[1][0],bbox[1][1]);
    for y in bbox[1][0]..bbox[1][1] {
        //for y in y_min to y_max of polygon bbox
        nodes = scanline_nodes(&poly, y as f64, width);
        if nodes.len() > 0 {
            scanline = Line::from_nodes(&nodes);
            rasterize_line_naive(&scanline, color, canvas);
            // println!("nodes: {:?}",nodes)
        }
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
        if p1y < scan_y && p2y >= scan_y || p1y >= scan_y && p2y < scan_y {
            p1x = edge.points[0][0];
            p2x = edge.points[1][0];

            if p1x < 0.0 {
                p1x = 0.0
            }
            scanline = Line {
                points: [[p1x, p1y], [p2x, p2y]],
            };
            line = Line {
                points: [[0.0, scan_y], [width, scan_y]],
            };

            node = scanline.line_intersection(&line);
            node[0] += 1; // accounts for line shifting left
            nodes.push(node)
        }
    }
    if nodes.len() > 0 {
        if nodes[0][1] != nodes[1][1] {
        }
    }
    nodes
}

pub fn rasterize_circle(point: &Point, radius: i32, color: [f32; 3], canvas: &mut Canvas) {
    let x = point.x.floor() as i32;
    let y = point.y.floor() as i32;
    let mut d;

    for _x in (x - radius)..(x + radius) {
        for _y in (y - radius)..(y + radius) {
            d = distance(&[x,y], &[_x,_y]);
            if d < radius as f64 {
                canvas.write_pixel(_x as usize, _y as usize, color);
            }
        }
    }
}
