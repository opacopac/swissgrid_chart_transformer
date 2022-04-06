use std::borrow::Borrow;
use std::time::SystemTime;
use image::GenericImageView;

use crate::geo::ch_1903_coord::Ch1903Coord;
use crate::geo::position_2d::Position2d;
use crate::imaging::drawable::Drawable;
use crate::imaging::image::{Image};
use crate::chart::ch_1903_chart::{Ch1903Chart};


mod geo;
mod imaging;
mod chart;


fn main() {
    let now = SystemTime::now();
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

    /*let extent = chart.calc_lat_lon_extent();
    println!("extent {}", now.elapsed().unwrap().as_millis());*/

    let proj = chart.calc_chart_projection();
    println!("projection {}", now.elapsed().unwrap().as_millis());

    proj.safe_image("OUT.png");
    println!("save {}", now.elapsed().unwrap().as_millis());
}
