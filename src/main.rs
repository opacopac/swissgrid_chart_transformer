use std::{error, thread, time};
use std::borrow::Borrow;
use std::error::Error;
use std::thread::Thread;
use std::time::SystemTime;

use clap::Parser;
use image::{GenericImageView, io};
use rayon::iter::split;
use simple_error::bail;

use crate::args::{ActionArg, Args, GeoRegMode};
use crate::chart::ch_1903_georeg_chart::Ch1903GeoRegChart;
use crate::chart::map_tile_projection_service::MapTileProjectionService;
use crate::chart::single_chart_projection_service::SingleChartProjectionService;
use crate::geo::ch_1903_coord::Ch1903Coord;
use crate::geo::coord::Coord;
use crate::geo::geo_reg::GeoReg;
use crate::geo::position_2d::Position2d;
use crate::geo::world_file_service::WorldFileService;
use crate::imaging::drawable::Drawable;
use crate::imaging::image::Image;

mod geo;
mod imaging;
mod chart;
mod args;

const SUBARG_SEPARATOR_CHAR: char = ',';


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let geo_reg = get_geo_reg(&args)?;
    let img = Image::load_img(&args.chart)?;
    let chart = Ch1903GeoRegChart::new(img, geo_reg);

    match args.action {
        ActionArg::Chart => {
            let output = SingleChartProjectionService::create_chart(&chart)?;
            output.drawable.safe_image(&args.output);
        },
        ActionArg::Tiles => {
            MapTileProjectionService::create_all_tiles(&chart, (0, 10), &args.output); // TODO
        }
    }

    let now = SystemTime::now();

    /*let input_file = "TMP_LSGG_AREA_DEP.png";
    let img = Image::load_img(input_file).unwrap();
    println!("loading {}", now.elapsed().unwrap().as_millis());

    let geo_reg = GeoReg::from_pos1_size_scale(
        (174.0, 1204.0),
        (466000.0, 104000.0),
        (img.width() as f32, img.height() as f32),
        250000.0,
        200.0
    );
    let chart = Ch1903GeoRegChart::new(img, geo_reg);
    println!("chart {}", now.elapsed().unwrap().as_millis());

    let proj = SingleChartProjectionService::create_chart(chart);
    println!("projection {}", now.elapsed().unwrap().as_millis());

    proj.drawable.safe_image("OUT.png");
    println!("save {}", now.elapsed().unwrap().as_millis());

    WorldFileService::save(proj.geo_reg, "OUT.pgw");
    let geo_reg2 = WorldFileService::read("OUT.pgw");*/


    /*let input_file = "luftfahrtkarten-icao_total_50_2056.tif";
    let img = Image::load_img(input_file).unwrap();
    println!("loading {}", now.elapsed().unwrap().as_millis());

    let geo_reg = WorldFileService::read("luftfahrtkarten-icao_total_50_2056.tfw");
    let chart = Ch1903GeoRegChart::new(img, geo_reg);
    println!("chart {}", now.elapsed().unwrap().as_millis());

    let base_path = ".";
    MapTileProjectionService::create_all_tiles(&chart, (0, 10), base_path);
    println!("tiles {}", now.elapsed().unwrap().as_millis());*/


    /*let input_file = "segelflugkarte_total_30_2056.png";
    let img = Image::load_img(input_file).unwrap();
    println!("loading {}", now.elapsed().unwrap().as_millis());

    let geo_reg = WorldFileService::read("segelflugkarte_total_30_2056.tfw")?;
    let chart = Ch1903GeoRegChart::new(img, geo_reg);
    println!("chart {}", now.elapsed().unwrap().as_millis());

    let base_path = ".";
    MapTileProjectionService::create_all_tiles(&chart, (0, 10), base_path);
    println!("tiles {}", now.elapsed().unwrap().as_millis());*/

    return Ok(());
}


fn get_geo_reg(args: &Args) -> Result<GeoReg, Box<dyn Error>> {
    return match &args.geo_reg_mode {
        GeoRegMode::Pos1Pos2Rot => {
            let parts = split_to_f32(&args.geo_reg_value, 8)?;

            Ok(GeoReg::from_pos1_pos2_rot(
                (parts[0], parts[1]),
                (parts[2], parts[3]),
                (parts[4], parts[5]),
                (parts[6], parts[7])
            ))
        },
        GeoRegMode::Pos1Pos2Stretch => {
            let parts = split_to_f32(&args.geo_reg_value, 8)?;

            Ok(GeoReg::from_pos1_pos2_stretch(
                (parts[0], parts[1]),
                (parts[2], parts[3]),
                (parts[4], parts[5]),
                (parts[6], parts[7])
            ))
        },
        GeoRegMode::Pos1SizeScale => {
            let parts = split_to_f32(&args.geo_reg_value, 8)?;

            Ok(GeoReg::from_pos1_size_scale(
                (parts[0], parts[1]),
                (parts[2], parts[3]),
                (parts[4], parts[5]),
                parts[6],
                parts[7]
            ))
        },
        GeoRegMode::WorldFile => {
            let geo_reg = WorldFileService::read(&args.geo_reg_value)?;

            Ok(geo_reg)
        },
    };
}


fn split_to_f32(text: &str, pieces: usize) -> Result<Vec<f32>, Box<dyn Error>> {
    let parts = text.split(SUBARG_SEPARATOR_CHAR)
        .map(|part| part.parse::<f32>().expect("argument must be a floating point value"))
        .collect::<Vec<_>>();

    if parts.len() != pieces {
        bail!("invalid number of arguments");
    }

    return Ok(parts);
}
