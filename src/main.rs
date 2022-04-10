use std::{thread, time};
use std::borrow::Borrow;
use std::thread::Thread;
use std::time::SystemTime;

use image::GenericImageView;

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


fn main() {
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


    /*let input_file = "luftfahrtkarten-icao_total_50_2056.png";
    let img = Image::load_img(input_file).unwrap();
    println!("loading {}", now.elapsed().unwrap().as_millis());

    let geo_reg = GeoReg::from_pos1_pos2_rot(
        (135.0, 4246.0),
        Ch1903Coord::from_lon_lat(5.5, 46.0).get_x_y(),
        (7751.0, 858.0),
        Ch1903Coord::from_lon_lat(10.5, 47.5).get_x_y(),
    );
    let chart = Ch1903GeoRegChart::new(img, geo_reg);
    println!("chart {}", now.elapsed().unwrap().as_millis());

    let base_path = ".";
    MapTileProjectionService::create_all_tiles(&chart, (0, 10), base_path);
    println!("tiles {}", now.elapsed().unwrap().as_millis());*/


    let input_file = "segelflugkarte_total_30_2056.png";
    let img = Image::load_img(input_file).unwrap();
    println!("loading {}", now.elapsed().unwrap().as_millis());

    let geo_reg = GeoReg::from_pos1_pos2_rot(
        (333.0, 7399.0),
        Ch1903Coord::new(490000.0, 80000.0).get_x_y(),
        (12000.0, 65.0),
        Ch1903Coord::new(840000.0, 300000.0).get_x_y(),
    );
    let chart = Ch1903GeoRegChart::new(img, geo_reg);
    println!("chart {}", now.elapsed().unwrap().as_millis());

    let base_path = ".";
    MapTileProjectionService::create_all_tiles(&chart, (0, 10), base_path);
    println!("tiles {}", now.elapsed().unwrap().as_millis());
}
