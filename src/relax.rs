use crate::canvas::Weighted_Canvas;
use crate::export::visualize_frame;
use crate::geometry::Unordered_Polygon;
use crate::seed::Seeds;
use crate::weighted_raster_centroid;
use display_utils::unicode_block_bar;
use std::io;
use std::io::Write; // <--- bring flush() into scope
use voronoi::{voronoi, Point};

pub fn lloyd_relax(
    start_seeds: &Seeds,
    iterations: u16,
    width: f64,
    image_path: &str,
    frames: bool,
) -> Vec<Point> {
    let mut seeds = start_seeds.clone();
    // TO DO: figure out what condition causes weighted_raster_centroid to return NaN

    //initializing variables
    let (mut poly, mut sorted_poly);
    let mut cR;
    let mut new_points;
    let mut faces;
    let mut vor_diagram;
    if frames {
        visualize_frame(
            0,
            &seeds,
            width as usize,
            width as usize,
            2,
            [1.0, 1.0, 1.0],
            [0.0, 0.0, 0.0],
        )
    }
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

            new_points.push(cR);
        }
        seeds.coords = new_points;

        // --------------
        // EXPORTING SEQUENCE
        // --------------

        if frames {
            visualize_frame(
                i + 1,
                &seeds,
                width as usize,
                width as usize,
                2,
                [1.0, 1.0, 1.0],
                [0.0, 0.0, 0.0],
            )
        }

        // ----------- Simple progress bar -----------
        print!("{esc}c", esc = 27 as char);
        println!("Performing relaxation iterations:\n");
        for _ in 0..i {
            print!("{}", unicode_block_bar(1, 1.0).to_string());
        }
        for _ in 0..(iterations - i) {
            print!(" ");
            io::stdout().flush().unwrap();
        }
        println!("({}/{})\n", i + 1, iterations);
        io::stdout().flush().unwrap();
        // -------------------------------------------

    }
    seeds.coords
}
