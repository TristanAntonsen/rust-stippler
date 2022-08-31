use crate::{seed::Seeds, geometry::point, rasterize::weighted_polygon_centroid};
use voronoi::{make_line_segments, make_polygons, voronoi, Point};
use crate::geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};

// pub fn lloyd_relax(start_points: &Seeds, iterations: u16, width: f64) -> Seeds {
//     let mut points = start_points.clone();
//     let start_seeds = points.coords.clone();
//     let mut vor_diagram = voronoi(start_seeds, width);

//     let mut new_seeds;


// }
