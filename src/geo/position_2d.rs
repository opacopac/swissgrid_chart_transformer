use std::fmt;
use crate::geo::coord::Coord;

pub struct Position2d {
    pub lon: f32,
    pub lat: f32,
}


impl fmt::Display for Position2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lon: {}, lat: {}", self.lon, self.lat)
    }
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
