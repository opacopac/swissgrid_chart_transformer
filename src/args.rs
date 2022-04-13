use std::str::FromStr;

use clap::{ArgEnum, Parser};


/// image chart transformer from swiss grid projection (LV03) to web mercator projection
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// input chart file (e.g. chart.tif)
    #[clap(short, long)]
    pub chart: String,

    /// 'world_file': filename of world file (e.g. chart.tfw)
    #[clap(short, long)]
    pub world_file: Option<String>,

    /// pos1_pos2_rot: TBD (e.g. 10,10,7.0,47.0,20,20,8.0,46.0)
    #[clap(short = 'r', long, number_of_values = 8)]
    pub pos1_pos2_rot: Option<Vec<f32>>,

    /// pos1_pos2_stretch: TBD
    #[clap(short = 's', long, number_of_values = 8)]
    pub pos1_pos2_stretch: Option<Vec<f32>>,

    /// pos1_size_scale: TBD
    #[clap(short = 't', long, number_of_values = 6)]
    pub pos1_size_scale: Option<Vec<f32>>,

    /// zoom levels for map tiles (e.g. 0,10)
    #[clap(short, long, number_of_values = 2)]
    pub zoom_range: Option<Vec<u8>>,

    /// output chart file or map tiles base directory
    #[clap(short, long)]
    pub output: String,
}
