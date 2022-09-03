use std::f32::consts::PI;

use nalgebra::Matrix1x3;
use voronoi::{Point, DCEL};
use radsort;
pub type point = [f64; 2];
pub type pixel = [i32; 2];
pub struct Ordered_Polygon {
    pub vertices: Vec<[f64; 2]>,
}

pub struct Unordered_Polygon {
    pub vertices: Vec<[f64; 2]>,
}

pub struct Line {
    pub points: [point; 2],
}

//return the nearest pixel as the floor of each floating point coordinate
pub fn nearest_pixel(point: &Point) -> pixel {
    [point.x.round() as i32, point.y.round() as i32]
}

pub fn distance(a: &pixel, b: &pixel) -> f64 {
    let _x = (b[0] - a[0]).pow(2) as f64;
    let _y = (b[1] - a[1]).pow(2) as f64;

    (_x + _y).sqrt()
}

pub fn vertex_centroid(points: &Vec<[f64; 2]>) -> [f64; 2] {
    let mut x = 0.0;
    let mut y = 0.0;
    let n = points.len();
    for point in points {
        x += point[0];
        y += point[1];
    }

    x /= n as f64;
    y /= n as f64;

    [x, y]
}

impl Ordered_Polygon {
    pub fn create_edges(&self) -> Vec<Line> {
        let n = self.vertices.len();
        let mut lines = Vec::new();
        for i in 0..(n - 1) {
            lines.push(Line {
                points: [self.vertices[i], self.vertices[i + 1]],
            })
        }
        lines.push(Line {
            points: [self.vertices[n - 1], self.vertices[0]],
        });

        lines
    }

    pub fn ngon(center: [f64; 2], radius: f64, sides: u8) -> Self {

        let mut vertices = Vec::new();
        let mut temp_point;
        let (mut x, mut y);
        let mut theta: f64 = 0.0;
        let inc = 2.0 * PI as f64 / sides as f64;


        for t in 0..sides {
            x = &radius * theta.cos();
            y = &radius * theta.sin();
            temp_point = [x + center[0], y + center[1]];
            vertices.push(temp_point);
            theta += inc;
        };

        Self {  
            vertices : vertices
        }

    }
}

impl Unordered_Polygon {

    pub fn from_face(face: &Vec<Point>) -> Self {
        //map the ordered floats into normal floats. Probably not ideal
        let vertices = face.iter()
        .map(|x| [f64::try_from(x.x).unwrap(),
        f64::try_from(x.y).unwrap()
        ]).collect();
        Self {
            vertices: vertices
        }
    }

    pub fn sort(&mut self) -> Ordered_Polygon {
        let mut sorted = Vec::new();
        let centroid = vertex_centroid(&self.vertices);
        let mut x;
        let mut y;
        let mut v_b;
        let mut numerator;
        let mut denominator;
        let mut theta;
        let mut _s;
        let mut sign;
        // let sorted_verts;

        let v_a = Matrix1x3::new(centroid[0] + 300.0, centroid[1], 0.0);
        for vert in &self.vertices {
            x = vert[0];
            y = vert[1];
            v_b = Matrix1x3::new(x - centroid[0], y - centroid[1], 0.0);

            numerator = v_a.dot(&v_b);
            denominator = v_a.magnitude() * v_b.magnitude();

            theta = (numerator / denominator).acos();
            _s = v_a.cross(&v_b);
            sign = _s / _s.magnitude();
            theta *= sign[2];
            sorted.push((vert, theta));
        }
        // let mut sorted = self.vertices.clone();
        radsort::sort_by_key(&mut sorted, |k| k.1);
        let verts = sorted.iter().map(|x| *x.0).collect::<Vec<[f64; 2]>>();

        Ordered_Polygon { vertices: verts }
    }
}


impl Line {
    pub fn from_halfedge(edge: usize, diagram: &DCEL) -> Self {
        let twin = &diagram.halfedges[edge].twin;
        let start = diagram.get_origin(edge);
        let end = diagram.get_origin(*twin);
        Self {
            points: [[*start.x, *start.y], [*end.x, *end.y]],
        }
    }
    pub fn from_segment(seg: &[Point; 2]) -> Self {
        Self {
            points: [[*seg[0].x, *seg[0].y], [*seg[1].x, *seg[1].y]],
        }
    }

    pub fn from_nodes(nodes: &Vec<pixel>) -> Self {
        //assumes exactly 2 nodes
        Self {
            points: [
                [nodes[0][0] as f64, nodes[0][1] as f64],
                [nodes[1][0] as f64, nodes[1][1] as f64],
            ],
        }
    }

    pub fn line_intersection(&self, other: &Line) -> pixel {
        let x1 = self.points[0][0];
        let y1 = self.points[0][1];
        let x2 = self.points[1][0];
        let y2 = self.points[1][1];

        let x3 = other.points[0][0];
        let y3 = other.points[0][1];
        let x4 = other.points[1][0];
        let y4 = other.points[1][1];

        let p_x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let p_x_denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let p_y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
        let p_y_denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        let p_x = p_x_num / p_x_denom;
        let p_y = p_y_num / p_y_denom;
        let intersect_point = Point::new(p_x, p_y);

        return nearest_pixel(&intersect_point);
    }
}