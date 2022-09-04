use std::path::PathBuf;
use structopt::StructOpt;

// // ARGUMENTS //
// let args: Vec<String> = env::args().collect();
// let image_path = &args[1];
// let points: usize = args[2].to_string().parse().unwrap();
// let iterations: u16 = args[3].to_string().parse().unwrap();
// let threshold: f32 = args[4].to_string().parse().unwrap();
// // --------- //

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct Opt {

    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
    
    /// Set point count
    #[structopt(short = "n", long = "point-count", default_value = "1000")]
    pub count: usize,


    // Set number of iterations
    #[structopt(short = "i", long = "iterations", default_value = "60")]
    pub iterations: u16,

    // Set threshold
    #[structopt(short = "t", long = "threshold", default_value = "0.5")]
    pub threshold: f32,

    // Save iteration images
    #[structopt(short = "f", long = "frames")]
    pub frames: bool,

}