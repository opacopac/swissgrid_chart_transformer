use std::{cmp, fmt, fs};
use std::iter::zip;
use std::sync::Mutex;

use min_max::{max, max_partial, min, min_partial};
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{Ch1903Coord, Ch1903GeoRegChart, Drawable, Image, Position2d};
use crate::geo::extent_2d::Extent2d;
use crate::geo::map_tile_coord::MapTileCoord;

pub struct CommonChartProjectionService;


impl CommonChartProjectionService {
    pub fn calc_lat_lon_extent(chart: &Ch1903GeoRegChart) -> Extent2d {
        let pos0 = chart.get_tl_coord().to_lon_lat();
        let mut min_lon = pos0.lon;
        let mut min_lat = pos0.lat;
        let mut max_lon = pos0.lon;
        let mut max_lat = pos0.lat;


        for x in 0..chart.width() {
            let pos1 = chart.calc_coord_by_pixel(x, 0).to_lon_lat();
            let pos2 = chart.calc_coord_by_pixel(x, chart.height() - 1).to_lon_lat();
            min_lon = min_partial!(pos1.lon, pos2.lon, min_lon);
            min_lat = min_partial!(pos1.lat, pos2.lat, min_lat);
            max_lon = max_partial!(pos1.lon, pos2.lon, max_lon);
            max_lat = max_partial!(pos1.lat, pos2.lat, max_lat);
        }

        for y in 0..chart.height() {
            let pos1 = chart.calc_coord_by_pixel(0, y).to_lon_lat();
            let pos2 = chart.calc_coord_by_pixel(chart.width() - 1, y).to_lon_lat();
            min_lon = min_partial!(pos1.lon, pos2.lon, min_lon);
            min_lat = min_partial!(pos1.lat, pos2.lat, min_lat);
            max_lon = max_partial!(pos1.lon, pos2.lon, max_lon);
            max_lat = max_partial!(pos1.lat, pos2.lat, max_lat);
        }

        return Extent2d {
            min_pos: Position2d { lon: min_lon, lat: min_lat },
            max_pos: Position2d { lon: max_lon, lat: max_lat }
        };
    }


    pub fn calc_area_projection(
        chart: &Ch1903GeoRegChart,
        px_width: u32,
        px_height: u32,
        min_pos_lon: f32,
        min_pos_lat: f32,
        lon_inc: f32,
        lat_inc: f32
    ) -> Vec<Vec<[u8; 4]>> {
        return (0..px_height).into_par_iter().map(|y| {
            let mut px_row: Vec<[u8; 4]> = Vec::new();

            for x in 0..px_width {
                let ch_coord = Ch1903Coord::from_lon_lat(
                    min_pos_lon + (x as f32) * lon_inc,
                    min_pos_lat + (y as f32) * lat_inc
                );
                px_row.push(chart.get_pixel_color(ch_coord));
            }

            return px_row;
        }).collect::<Vec<Vec<[u8; 4]>>>();
    }
}
