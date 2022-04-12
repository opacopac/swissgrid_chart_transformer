use std::f32::consts::PI;
use std::fmt;

use crate::Position2d;

#[derive(Debug)]
pub struct MapTileCoord {
    pub x: u32,
    pub y: u32,
    pub zoom: u32
}


impl MapTileCoord {
    pub const TILE_SIZE_PX: u32 = 256;


    pub fn new(x_tile: u32, y_tile: u32, zoom: u32) -> MapTileCoord {
        return MapTileCoord { x: x_tile, y: y_tile, zoom };
    }


    pub fn from_position(pos: &Position2d, zoom: u32) -> MapTileCoord {
        let pow = 2_u32.pow(zoom) as f32;
        let x_tile = ((pos.lon + 180.0) / 360.0 * pow).floor() as u32;
        let y_tile = ((1.0 - (pos.lat.to_radians().tan() + 1.0 / pos.lat.to_radians().cos()).ln() / PI) / 2.0 * pow).floor() as u32;

        return MapTileCoord { x: x_tile, y: y_tile, zoom };
    }


    pub fn to_position(&self) -> Position2d {
        let n = 2_u32.pow(self.zoom) as f32;
        let lon = self.x as f32 / n * 360.0 - 180.0;
        let lat = (((PI * (1.0 - 2.0 * self.y as f32 / n)).sinh()).atan()).to_degrees();

        return Position2d { lon, lat }
    }
}
