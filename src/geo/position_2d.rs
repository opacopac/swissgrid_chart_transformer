use std::fmt;

use crate::geo::coord::Coord;

#[derive(Debug)]
pub struct Position2d {
    pub lon: f32,
    pub lat: f32,
}


impl Coord for Position2d {
    fn get_x_y(&self) -> (f32, f32) {
        return (self.lon, self.lat);
    }
}


impl Position2d {
    pub fn new(lon: f32, lat: f32) -> Position2d {
        return Position2d { lon, lat };
    }
}
