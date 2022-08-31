use crate::{seed::Seeds, geometry::point, rasterize::weighted_polygon_centroid};
use voronoi::{make_line_segments, make_polygons, voronoi, Point};
use crate::geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use crate::canvas::Weighted_Canvas;

// pub fn lloyd_relax(seeds: &Seeds, iterations: u16, width: f64, image_path: &str) -> Seeds {

//     //create voronoi diagram
//     let vor_diagram = voronoi(seeds.coords, width as f64);

//     //faces of diagram
//     let faces = voronoi::make_polygons(&vor_diagram);
    
//     //initializing variables
//     let (mut poly, mut sorted_poly);
//     let (mut _c, mut cV, mut cR, mut color);

//     //creating weight array (grayscale)
//     let mut weights = Weighted_Canvas::from_image(image_path);


// }
