use std::fmt;
use crate::geo::position_2d::Position2d;


pub struct Extent2d {
    pub min_pos: Position2d,
    pub max_pos: Position2d
}


impl fmt::Display for Extent2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "min_pos: {}, max_pos: {}", self.min_pos, self.max_pos)
    }
}


impl Extent2d {
    pub fn calc_mid_pos(&self) -> Position2d {
        return Position2d {
            lon: (self.min_pos.lon + self.max_pos.lon) / 2.0,
            lat: (self.min_pos.lat + self.max_pos.lat) / 2.0
        };
    }
}
