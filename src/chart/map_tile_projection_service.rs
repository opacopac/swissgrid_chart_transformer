use std::fs;

use min_max::{max, min};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{Ch1903GeoRegChart, Drawable, Position2d};
use crate::chart::common_chart_projection_service::CommonChartProjectionService;
use crate::geo::map_tile_coord::MapTileCoord;

pub struct MapTileProjectionService;


impl MapTileProjectionService {
    pub fn create_all_tiles(
        chart: &Ch1903GeoRegChart,
        zoom_range: (u32, u32),
        base_path: &str
    ) {
        let extent = CommonChartProjectionService::calc_lat_lon_extent(chart);

        let pos_tl = Position2d::new(extent.min_pos.lon, extent.max_pos.lat);
        let pos_br = Position2d::new(extent.max_pos.lon, extent.min_pos.lat);

        for zoom in zoom_range.0..=zoom_range.1 {
            let tile_tl = MapTileCoord::from_position(&pos_tl, zoom);
            let tile_br = MapTileCoord::from_position(&pos_br, zoom);
            let x_range = (min(tile_tl.x, tile_br.x), max(tile_tl.x, tile_br.x));
            let y_range = (min(tile_tl.y, tile_br.y), max(tile_tl.y, tile_br.y));

            for x in x_range.0..=x_range.1 {
                (y_range.0..=y_range.1).into_par_iter().for_each(|y| {
                    //println!("rendering tile x: {}, y: {}, z: {}", x, y, zoom);
                    let tile = MapTileProjectionService::create_single_tile(&chart, zoom, x, y);
                    MapTileProjectionService::save_tile(&tile, zoom, x, y, base_path);
                })
            }
        }
    }


    fn create_single_tile(
        chart: &Ch1903GeoRegChart,
        zoom: u32,
        x: u32,
        y: u32
    ) -> Drawable {
        let tile_size_px = MapTileCoord::TILE_SIZE_PX as f32;
        let tile_coord_min = MapTileCoord::new(x, y, zoom);
        let tile_coord_max = MapTileCoord::new(x + 1, y + 1, zoom);
        let min_lon = tile_coord_min.to_position().lon;
        let min_lat = tile_coord_min.to_position().lat;
        let max_lon = tile_coord_max.to_position().lon;
        let max_lat = tile_coord_max.to_position().lat;
        let lon_inc = (max_lon - min_lon) / tile_size_px;
        let lat_inc = (max_lat - min_lat) / tile_size_px;

        let px_rows = CommonChartProjectionService::calc_area_projection(
            &chart,
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


    fn save_tile(
        tile: &Drawable,
        zoom: u32,
        x: u32,
        y: u32,
        base_path: &str
    ) {
        let path = format!("{}/{}/{}", base_path, zoom, x);
        fs::create_dir_all(&path).unwrap();

        let filename = format!("{}/{}.png", &path, y);
        let result = tile.safe_image(&filename);
    }
}
