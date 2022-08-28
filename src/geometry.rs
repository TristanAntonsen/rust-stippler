use std::ops::Sub;

use voronoi::{Point, DCEL};

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

// impl Unordered_Polygon {
//     pub fn sort(&self) -> Ordered_Polygon {
//         let angles = Vec::new();
//         let centroid = 
//     }
// }

// def Sort_Vertices(polygon):
//     angles = []
//     centroid = Calculate_Centroid(polygon)
//     # vA = x vector starting at centroid
//     vA = np.array([centroid[0] + 300,centroid[1],0])
//     s = 5
//     for vert in polygon:
//         x = vert[0]
//         y = vert[1]
//         #vB = vector between centroid and vertex
//         vB = np.array([x - centroid[0], y - centroid[1],0])
        
//         numerator = np.dot(vA,vB)
//         denominator = np.linalg.norm(vA) * np.linalg.norm(vB)
//         theta = np.arccos(numerator / denominator)

//         sign = np.cross(vA,vB) / np.linalg.norm(np.cross(vA,vB))
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
