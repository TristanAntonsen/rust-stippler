use voronoi::Point;

pub type point = [f64; 2];
pub type pixel = [usize; 2];
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
    [point.x.floor() as usize, point.y.floor() as usize]
}

// pub fn extrac