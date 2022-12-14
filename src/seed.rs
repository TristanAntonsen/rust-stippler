use crate::geometry::nearest_pixel;
use crate::{Canvas,Weighted_Canvas};
use rand::Rng;
use rand::distributions::Uniform;
use voronoi::Point;

#[derive(Debug, Clone)]
pub struct Seeds {
    pub coords : Vec<Point>
}

impl Seeds {
    pub fn uniform(canvas: &Canvas, count: usize) -> Self {
        let width = canvas.pixels[0].len();
        let height = canvas.pixels.len();

        let mut rng = rand::thread_rng();
        let range1 = Uniform::new(0., width as f64);
        let range2 = Uniform::new(0., height as f64);
        let seeds: Vec<Point> = (0..count)
        .map(|_| Point::new(rng.sample(&range1), rng.sample(&range2)))
        .collect();
        Self {
            coords: seeds
        }
    }

    pub fn cartesian(canvas: &Weighted_Canvas, spacing: f64, threshold: f32) -> Self {
        let width = canvas.pixel_weights[0].len();
        let height = canvas.pixel_weights.len();
        let mut seeds = Vec::new();
        let (mut point, mut pixel, mut sampled_value);
        let x_count = (width as f64 / spacing).round() as u32;
        let y_count = (height as f64 / spacing).round() as u32;
        let x_inc = width as u32 / x_count;
        let y_inc = height as u32 / y_count;

        for x in 1..x_count {
            for y in 1..y_count {
                point = Point::new((x * x_inc) as f64,(y * y_inc) as f64);
                pixel = nearest_pixel(&point);
                sampled_value = canvas.read_pixel(
                    point.x.round() as usize,
                    point.y.round() as usize
                );
                if sampled_value < threshold {
                    seeds.push(point);
                }
            }
        }

        Self {
            coords: seeds
        }
    }


    pub fn rejection_sample(weights: &Weighted_Canvas, count: usize, threshold: f32) -> Self {
        let width = weights.pixel_weights[0].len();
        let height = weights.pixel_weights.len();
        let mut seeds: Vec<Point> = Vec::new();
        let mut x: f64;
        let mut y: f64;
        let mut rng = rand::thread_rng();
        let mut i = 0;
        let mut point = Point::new(0.0,0.0);
        let mut pixel;
        let mut sampled_value;

        while i < count {
            x = rng.gen::<f64>() * width as f64;
            y = rng.gen::<f64>() * height as f64;
            pixel = nearest_pixel(&point);
            point = Point::new(x,y);
            sampled_value = weights.read_pixel(
                point.x.round() as usize,
                point.y.round() as usize
            );
            if sampled_value < threshold {
                seeds.push(point);
                i += 1;
            }

        }
        Seeds { coords: seeds }


    }

    pub fn pdf_rejection_sample(weights: &Weighted_Canvas, count: usize, threshold: f32) -> Self {

        // probability of placing seed = 1 / (pdf(grayscale value))


        let width = weights.pixel_weights[0].len();
        let height = weights.pixel_weights.len();
        let mut seeds: Vec<Point> = Vec::new();
        let mut x: f64;
        let mut y: f64;
        let mut rng = rand::thread_rng();
        let mut i = 0;
        let mut point = Point::new(0.0,0.0);
        let mut pixel;
        let mut sampled_value;

        // pdf parameters
        let scalar = 10.0;
        let mut probability;
        let mut _p;


        while i < count {
            x = rng.gen::<f64>() * width as f64;
            y = rng.gen::<f64>() * height as f64;
            
            pixel = nearest_pixel(&point);
            point = Point::new(x,y);
            sampled_value = weights.read_pixel(
                point.x.round() as usize,
                point.y.round() as usize
            );

            _p = rng.gen::<f64>() as f32;
            println!("{}",_p);
            probability = _p * (sampled_value.powf(2.0));
            println!("{}",probability);

            if probability < threshold {
                seeds.push(point);
                i += 1;
            }

        }
        Seeds { coords: seeds }


    }

}