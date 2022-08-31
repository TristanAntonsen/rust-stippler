use crate::canvas::Weighted_Canvas;
use crate::geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use crate::{geometry::point, rasterize::weighted_raster_centroid, seed::Seeds};
use voronoi::{make_line_segments, make_polygons, voronoi, Point};

pub fn lloyd_relax(start_seeds: &Seeds,iterations: u16,width: f64,image_path: &str) -> Vec<Point> {
    let mut seeds = start_seeds.clone();
    // TO DO: figure out what condition causes weighted_raster_centroid to return NaN

    //initializing variables
    let (mut poly, mut sorted_poly);
    let mut cR;
    let mut new_points;
    let mut faces;
    let mut vor_diagram;

    for _ in 0..iterations {
        //create voronoi diagram
        println!("{:?}", seeds.coords);
        vor_diagram = voronoi(seeds.coords, width as f64);
        //faces of diagram
        faces = voronoi::make_polygons(&vor_diagram);
        //creating weight array (grayscale)
        let mut weights = Weighted_Canvas::from_image(image_path);
        new_points = Vec::new();
        for face in faces {
            //creating unordered polygon from region
            poly = Unordered_Polygon::from_face(&face);
            // sorting ordered polygon
            sorted_poly = poly.sort();

            //creating the weighted centroid of the polygon
            cR = weighted_raster_centroid(&sorted_poly, &mut weights);

            new_points.push(cR);
        }
        seeds.coords = new_points;
        println!("DONE");
    }

    seeds.coords
}
