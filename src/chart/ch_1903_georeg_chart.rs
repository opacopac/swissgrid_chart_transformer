use crate::{Ch1903Coord, Image};
use crate::geo::coord::Coord;
use crate::geo::geo_reg::GeoReg;


pub struct Ch1903GeoRegChart {
    image: Image,
    geo_reg: GeoReg,
}


impl Ch1903GeoRegChart {
    pub fn new(
        image: Image,
        geo_reg: GeoReg
    ) -> Ch1903GeoRegChart {
        return Ch1903GeoRegChart {
            image,
            geo_reg
        };
    }


    pub fn width(&self) -> u32 {
        return self.image.width();
    }


    pub fn height(&self) -> u32 {
        return self.image.height();
    }


    pub fn get_tl_coord(&self) -> Ch1903Coord {
        return self.calc_coord_by_pixel(0, 0);
    }


    pub fn get_br_coord(&self) -> Ch1903Coord {
        return self.calc_coord_by_pixel(self.image.width() - 1, self.image.height() - 1);
    }


    pub fn calc_coord_by_pixel(&self, x: u32, y: u32) -> Ch1903Coord {
        let coord = self.geo_reg.calc_coord_by_px(x as f32, y as f32);

        return Ch1903Coord::new(coord.0, coord.1);
    }


    pub fn get_pixel_color(&self, coord: Ch1903Coord) -> [u8; 4] {
        let px = self.geo_reg.calc_px_by_coord(coord.get_x_y());

        return self.image.interpolate_pixel_color(px.0, px.1);
    }
}
