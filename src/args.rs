use std::str::FromStr;

use clap::{ArgEnum, Parser};

#[derive(clap::ArgEnum, Clone, Debug)]
pub enum ActionArg {
    Chart,
    Tiles
}

impl FromStr for ActionArg {
    type Err = String;
    fn from_str(action: &str) -> Result<Self, Self::Err> {
        match action {
            "chart" => Ok(ActionArg::Chart),
            "tiles" => Ok(ActionArg::Tiles),
            _ => Err("unknown action".to_string()),
        }
    }
}


#[derive(clap::ArgEnum, Clone, Debug)]
pub enum GeoRegMode {
    WorldFile,
    Pos1Pos2Rot,
    Pos1Pos2Stretch,
    Pos1SizeScale
}


impl FromStr for GeoRegMode {
    type Err = String;
    fn from_str(action: &str) -> Result<Self, Self::Err> {
        match action {
            "world_file" => Ok(GeoRegMode::WorldFile),
            "pos1_pos2_rot" => Ok(GeoRegMode::Pos1Pos2Rot),
            "pos1_pos2_stretch" => Ok(GeoRegMode::Pos1Pos2Stretch),
            "pos1_size_scale" => Ok(GeoRegMode::Pos1SizeScale),
            _ => Err("unknown action".to_string()),
        }
    }
}


/// image chart transformer from swiss grid projection (LV03) to web mercator projection
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// input chart file (e.g. chart.tif)
    #[clap(short, long)]
    pub chart: String,

    /// geo registration mode [ world_file | pos1_pos2_rot | pos1_pos2_stretch | pos1_size_scale]
    #[clap(short = 'g', long)]
    pub geo_reg_mode: GeoRegMode,

    /// geo registration value
    ///  mode 'world_file': filename of world file (e.g. chart.tfw)
    ///  mode 'pos1_pos2_rot': TBD (e.g. 10,10,7.0,47.0,20,20,8.0,46.0)
    ///  mode 'pos1_pos2_stretch': TBD
    ///  mode 'pos1_size_scale': TBD
    #[clap(short = 'w', long)]
    pub geo_reg_value: String,

    /// action to perform [chart | tiles]
    #[clap(short, long, default_value = "chart")]
    pub action: ActionArg,

    /// output chart file or map tiles base directory
    #[clap(short, long)]
    pub output: String,
}
