use nalgebra::Matrix1x3;
use core::num;
use std::ops::Sub;
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
    [point.x.floor() as i32, point.y.floor() as i32]
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
}

impl Unordered_Polygon {
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

// def Sort_Vertices(polygon):
//     angles = []
//     centroid = Calculate_Centroid(polygon)
//     # v_a = x vector starting at centroid
//     v_a = np.array([centroid[0] + 300,centroid[1],0])
//     s = 5
//     for vert in polygon:
//         x = vert[0]
//         y = vert[1]
//         #v_b = vector between centroid and vertex
//         v_b = np.array([x - centroid[0], y - centroid[1],0])

//         numerator = np.dot(v_a,v_b)
//         denominator = np.linalg.norm(v_a) * np.linalg.norm(v_b)
//         theta = np.arccos(numerator / denominator)

//         sign = np.cross(v_a,v_b) / np.linalg.norm(np.cross(v_a,v_b))
//         theta *= sign[2]

//         angles.append(theta)

//     sorted_verts = np.array([x for _, x in sorted(zip(angles, polygon))])

//     return sorted_verts

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

// def Line_Intersection(p1, p2, p3, p4):

//     ## Line 1
//     x1 = p1[0]
//     y1 = p1[1]
//     x2 = p2[0]
//     y2 = p2[1]

//     ## Line 2
//     x3 = p3[0]
//     y3 = p3[1]
//     x4 = p4[0]
//     y4 = p4[1]

//     ## p_x

//     p_x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)
//     p_x_denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)

//     p_y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)
//     p_y_denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)

//     p_x = p_x_num / p_x_denom
//     p_y = p_y_num / p_y_denom

//     return (p_x, p_y)
