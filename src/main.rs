use std::{error, thread, time};
use std::borrow::Borrow;
use std::error::Error;
use std::thread::Thread;
use std::time::SystemTime;

use clap::Parser;
use image::{GenericImageView, io};
use rayon::iter::split;
use simple_error::bail;

use crate::args::Args;
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


fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let geo_reg = get_geo_reg(&args)?;
    let img = Image::load_img(&args.chart)?;
    let chart = Ch1903GeoRegChart::new(img, geo_reg);

    match args.zoom_range {
        None => {
            let output = SingleChartProjectionService::create_chart(&chart)?;
            output.drawable.safe_image(&args.output);
        },
        Some(zoom_levels) => {
            MapTileProjectionService::create_all_tiles(
                &chart,
                (zoom_levels[0] as u32, zoom_levels[1] as u32),
                &args.output
            );
        }
    }

    /*let now = SystemTime::now();

    let input_file = "TMP_LSGG_AREA_DEP.png";
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
    match &args.world_file {
        Some(value) => {
            let geo_reg = WorldFileService::read(value)?;

            return Ok(geo_reg);
        },
        None => {}
    }

    match &args.pos1_pos2_rot {
        Some(values) => {
            return Ok(GeoReg::from_pos1_pos2_rot(
                (values[0], values[1]),
                (values[2], values[3]),
                (values[4], values[5]),
                (values[6], values[7])
            ));
        },
        None => {}
    }

    match &args.pos1_pos2_stretch {
        Some(values) => {
            return Ok(GeoReg::from_pos1_pos2_stretch(
                (values[0], values[1]),
                (values[2], values[3]),
                (values[4], values[5]),
                (values[6], values[7])
            ));
        },
        None => {}
    }

    match &args.pos1_size_scale {
        Some(values) => {
            return Ok(GeoReg::from_pos1_size_scale(
                (values[0], values[1]),
                (values[2], values[3]),
                values[4],
                values[5]
            ));
        },
        None => {}
    }

    bail!("no geo registration provided")
}
