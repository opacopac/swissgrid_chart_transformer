use std::fmt;
use crate::geo::position_2d::Position2d;


#[derive(Debug)]
pub struct Extent2d {
    pub min_pos: Position2d,
    pub max_pos: Position2d
}


impl Extent2d {
    pub fn calc_mid_pos(&self) -> Position2d {
        return Position2d {
            lon: (self.min_pos.lon + self.max_pos.lon) / 2.0,
            lat: (self.min_pos.lat + self.max_pos.lat) / 2.0
        };
    }
}
