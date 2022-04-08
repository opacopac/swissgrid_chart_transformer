use min_max::{max, max_partial, min, min_partial};
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::fmt;
use std::sync::Mutex;

use crate::{Ch1903Chart, Ch1903Coord, Drawable, Image, Position2d};
use crate::geo::extent_2d::Extent2d;
use crate::geo::map_tile_coord::MapTileCoord;


pub struct ChartProjector;


impl ChartProjector {
    pub fn project_full_chart(chart: Ch1903Chart) -> Drawable {
        let extent = ChartProjector::calc_lat_lon_extent(&chart);
        let mid_pos = extent.calc_mid_pos();
        let lon_diff = extent.max_pos.lon - extent.min_pos.lon;
        let lat_diff = extent.max_pos.lat - extent.min_pos.lat;
        let px_width = chart.width();
        let px_per_deg = px_width as f32 / lon_diff;
        let lat_rad = mid_pos.lat.to_radians();
        let px_height = (lat_diff * px_per_deg / lat_rad.cos()).round() as u32;
        let lon_inc = lon_diff / (px_width as f32 - 1.0);
        let lat_inc = lat_diff / (px_height as f32 - 1.0);

        let px_rows = ChartProjector::project_pixel_rows(
            chart,
            px_width,
            px_height,
            extent.min_pos.lon,
            extent.min_pos.lat,
            lon_inc,
            lat_inc
        );
        let drawable = Drawable::create_with_data(px_width, px_height, px_rows).unwrap();

        return drawable;
    }


    pub fn project_map_tile(chart: Ch1903Chart, zoom: u32, x: u32, y: u32) -> Drawable {
        let tile_size_px = MapTileCoord::TILE_SIZE_PX as f32;
        let tile_coord_min = MapTileCoord::new(x, y, zoom);
        let tile_coord_max = MapTileCoord::new(x + 1, y + 1, zoom);
        let min_lon = tile_coord_min.to_position().lon;
        let min_lat = tile_coord_min.to_position().lat;
        let max_lon = tile_coord_max.to_position().lon;
        let max_lat = tile_coord_max.to_position().lat;
        let lon_inc = (max_lon - min_lon) / tile_size_px;
        let lat_inc = (max_lat - min_lat) / tile_size_px;

        let px_rows = ChartProjector::project_pixel_rows(
            chart,
            MapTileCoord::TILE_SIZE_PX,
            MapTileCoord::TILE_SIZE_PX,
            min_lon,
            max_lat - lat_inc,
            lon_inc,
            -lat_inc
        );
        let drawable = Drawable::create_with_data(MapTileCoord::TILE_SIZE_PX, MapTileCoord::TILE_SIZE_PX, px_rows).unwrap();

        return drawable;
    }


    fn calc_lat_lon_extent(chart: &Ch1903Chart) -> Extent2d {
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


    fn project_pixel_rows(
        chart: Ch1903Chart,
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
