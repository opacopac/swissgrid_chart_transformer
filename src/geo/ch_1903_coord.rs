use std::fmt;
use crate::geo::coord::Coord;
use crate::geo::position_2d::Position2d;


#[derive(Debug)]
pub struct Ch1903Coord {
    pub e: f32,
    pub n: f32
}


impl Coord for Ch1903Coord {
    fn get_x_y(&self) -> (f32, f32) {
        return (self.e, self.n);
    }
}


impl Ch1903Coord {
    pub fn new(e: f32, n: f32) -> Ch1903Coord {
        return Ch1903Coord { e, n };
    }


    pub fn from_pos2d(pos2d: Position2d) -> Ch1903Coord {
        let e = Ch1903Coord::wgs_to_ch_y(pos2d.lat, pos2d.lon);
        let n = Ch1903Coord::wgs_to_ch_x(pos2d.lat, pos2d.lon);

        return Ch1903Coord { e, n };
    }


    pub fn from_lon_lat(lon: f32, lat: f32) -> Ch1903Coord {
        let e = Ch1903Coord::wgs_to_ch_y(lat, lon);
        let n = Ch1903Coord::wgs_to_ch_x(lat, lon);

        return Ch1903Coord { e, n };
    }


    pub fn to_lon_lat(&self) -> Position2d {
        let lon = Ch1903Coord::ch_to_wgs_long(self.e, self.n);
        let lat = Ch1903Coord::ch_to_wgs_lat(self.e, self.n);

        return Position2d { lon, lat };
    }


    // Convert WGS lat/long (° dec) to CH y
    fn wgs_to_ch_y(lat: f32, long: f32) -> f32 {

        // Converts decimal degrees sexagesimal seconds
        let lat = Ch1903Coord::dec_to_sex(lat);
        let long = Ch1903Coord::dec_to_sex(long);

        // Auxiliary values (% Bern)
        let lat_aux = (lat - 169028.66) / 10000.0;
        let long_aux = (long - 26782.5) / 10000.0;

        // Process Y
        let y = 600072.37
            + 211455.93 * long_aux
            - 10938.51 * long_aux * lat_aux
            - 0.36 * long_aux * lat_aux * lat_aux
            - 44.54 * long_aux * long_aux * long_aux;

        return y;
    }


    // Convert WGS lat/long (° dec) to CH x
    fn wgs_to_ch_x(lat: f32, long: f32) -> f32 {

        // Converts decimal degrees sexagesimal seconds
        let lat = Ch1903Coord::dec_to_sex(lat);
        let long = Ch1903Coord::dec_to_sex(long);

        // Auxiliary values (% Bern)
        let lat_aux = (lat - 169028.66) / 10000.0;
        let long_aux = (long - 26782.5) / 10000.0;

        // Process X
        let x = 200147.07
            + 308807.95 * lat_aux
            + 3745.25 * long_aux * long_aux
            + 76.63 * lat_aux * lat_aux
            - 194.56 * long_aux * long_aux * lat_aux
            + 119.79 * lat_aux * lat_aux * lat_aux;

        return x;
    }


    // Convert CH y/x to WGS lat
    fn ch_to_wgs_lat(y: f32, x: f32) -> f32 {

        // Converts military to civil and  to unit = 1000km
        // Auxiliary values (% Bern)
        let y_aux = (y - 600000.0) / 1000000.0;
        let x_aux = (x - 200000.0) / 1000000.0;

        // Process lat
        let lat = 16.9023892
            + 3.238272 * x_aux
            - 0.270978 * y_aux * y_aux
            - 0.002528 * x_aux * x_aux
            - 0.0447 * y_aux * y_aux * x_aux
            - 0.0140 * x_aux * x_aux * x_aux;

        // Unit 10000" to 1 " and converts seconds to degrees (dec)
        let lat = lat * 100.0 / 36.0;

        return lat;
    }


    // Convert CH y/x to WGS long
    fn ch_to_wgs_long(y: f32, x: f32) -> f32 {

        // Converts military to civil and  to unit = 1000km
        // Auxiliary values (% Bern)
        let y_aux = (y - 600000.0) / 1000000.0;
        let x_aux = (x - 200000.0) / 1000000.0;

        // Process long
        let long = 2.6779094
            + 4.728982 * y_aux
            + 0.791484 * y_aux * x_aux
            + 0.1306 * y_aux * x_aux * x_aux
            - 0.0436 * y_aux * y_aux * y_aux;

        // Unit 10000" to 1 " and converts seconds to degrees (dec)
        let long = long * 100.0 / 36.0;

        return long;
    }


    fn dec_to_sex(angle: f32) -> f32 {
        // Extract DMS
        let deg = f32::floor(angle);
        let min = f32::floor((angle - deg) * 60.0);
        let sec = (((angle - deg) * 60.0) - min) * 60.0;

        // Result in sexagesimal seconds
        return sec + min * 60.0 + deg * 3600.0;
    }
}
