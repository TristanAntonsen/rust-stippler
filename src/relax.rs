use crate::canvas::{Weighted_Canvas, Canvas};
use crate::geometry::{vertex_centroid, Line, Ordered_Polygon, Unordered_Polygon};
use crate::{geometry::point, seed::Seeds};
use crate::{weighted_raster_centroid, rasterize_circle};
use voronoi::{make_line_segments, make_polygons, voronoi, Point};
use crate::export::{save_image, visualize_frame};


pub fn lloyd_relax(start_seeds: &Seeds,iterations: u16, width: f64, image_path: &str) -> Vec<Point> {
    let mut seeds = start_seeds.clone();
    // TO DO: figure out what condition causes weighted_raster_centroid to return NaN

    //initializing variables
    let (mut poly, mut sorted_poly);
    let mut cR;
    let mut new_points;
    let mut faces;
    let mut vor_diagram;
    for i in 0..iterations {
        //create voronoi diagram
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
            // cR = vertex_centroid(&sorted_poly.vertices,);
            // println!("cR: {:?}",cR);
            new_points.push(cR);
            // new_points.push(Point::new(cR[0],cR[1]));
        }
        seeds.coords = new_points;

        // --------------
        // EXPORTING SEQUENCE
        // --------------
        // let mut canvas = Canvas::solid_color(width as usize, width as usize,[1.0,1.0,1.0]);
        // let mut file_name = "sequence/".to_string();
        // for point in &seeds.coords {
        //     rasterize_circle(&point, 3, [0.0, 0.0, 0.0], &mut canvas)
        // }
        
        // file_name.push_str(&i.to_string());
        // file_name.push_str(".jpg");
        // save_image(&file_name[..], canvas);
        // println!("iteration {}",i);
        // visualize_frame(i, &seeds, width as usize, width as usize, 2, [1.0,1.0,1.0], [0.0,0.0,0.0])
    }

    seeds.coords
}
