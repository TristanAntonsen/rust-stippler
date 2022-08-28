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
}