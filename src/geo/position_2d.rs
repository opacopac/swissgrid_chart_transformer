use std::fmt;

pub struct Position2d {
    pub lon: f32,
    pub lat: f32,
}


impl fmt::Display for Position2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lon: {}, lat: {}", self.lon, self.lat)
    }
}
