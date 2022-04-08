use std::borrow::Borrow;
use std::{thread, time};
use std::thread::Thread;
use std::time::SystemTime;

use image::GenericImageView;

use crate::chart::ch_1903_chart::Ch1903Chart;
use crate::chart::chart_projector::ChartProjector;
use crate::geo::ch_1903_coord::Ch1903Coord;
use crate::geo::position_2d::Position2d;
use crate::imaging::drawable::Drawable;
use crate::imaging::image::Image;

mod geo;
mod imaging;
mod chart;


fn main() {
    /*let now = SystemTime::now();
    let input_file = "TMP_LSGG_AREA_DEP.png";
    let img = Image::load_img(input_file).unwrap();

    println!("loading {}", now.elapsed().unwrap().as_millis());

    let chart = Ch1903Chart::from_pos_and_scale(
        img,
        (174, 1204),
        Ch1903Coord { e: 466000.0, n: 104000.0 },
        250000.0,
        200.0
    );
    println!("chart {}", now.elapsed().unwrap().as_millis());

    let proj = ChartProjector::project_full_chart(chart);
    println!("projection {}", now.elapsed().unwrap().as_millis());

    proj.safe_image("OUT.png");
    println!("save {}", now.elapsed().unwrap().as_millis());*/


    let now = SystemTime::now();

    /*let input_file = "luftfahrtkarten-icao_total_50_2056.png";
    let img = Image::load_img(input_file).unwrap();
    println!("loading {}", now.elapsed().unwrap().as_millis());

    let chart = Ch1903Chart::from_pos1_and_pos2(
        img,
        (135, 4246),
        Ch1903Coord::from_lon_lat(5.5, 46.0),
        (7751, 858),
        Ch1903Coord::from_lon_lat(10.5, 47.5),
    );
    println!("chart {}", now.elapsed().unwrap().as_millis());*/


    let input_file = "segelflugkarte_total_30_2056.png";
    let img = Image::load_img(input_file).unwrap();
    println!("loading {}", now.elapsed().unwrap().as_millis());

    let chart = Ch1903Chart::from_pos1_and_pos2(
        img,
        (333, 7399),
        Ch1903Coord::new(490000.0, 80000.0),
        (12000, 65),
        Ch1903Coord::new(840000.0, 300000.0),
    );
    println!("chart {}", now.elapsed().unwrap().as_millis());


    let proj = ChartProjector::project_map_tile(&chart, 11, 1067, 722);
    println!("projection {}", now.elapsed().unwrap().as_millis());

    proj.safe_image("OUT_tile.png");
    println!("save {}", now.elapsed().unwrap().as_millis());

    ChartProjector::create_zoomlevel_tiles(&chart, 13);
    println!("tiles {}", now.elapsed().unwrap().as_millis());
}
