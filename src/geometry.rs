use voronoi::{Point, DCEL};

pub type point = [f64; 2];
pub type pixel = [i32; 2];
pub struct Ordered_Polygon {
    pub vertices: Vec<[f32; 2]>,
}

pub struct Unordered_Polygon {
    pub vertices: Vec<[f32; 2]>,
}


pub struct Line {
    pub points: [point; 2],
}

//return the nearest pixel as the floor of each floating point coordinate
pub fn nearest_pixel(point: &Point) -> pixel {
    [point.x.floor() as i32, point.y.floor() as i32]
}

// pub fn extrac

impl Line {
    pub fn from_halfedge(edge: usize, diagram: &DCEL) -> Self {
        let twin = &diagram.halfedges[edge].twin;
        let start = diagram.get_origin(edge);
        let end = diagram.get_origin(*twin);
        Self {
            points: [[*start.x, *start.y],[*end.x, *end.y] ]
        }
    }
    pub fn from_segment(seg: [Point; 2]) -> Self {
        Self {
            points: [[*seg[0].x, *seg[0].y],[*seg[1].x, *seg[1].y]]
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

        return nearest_pixel(&intersect_point)

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