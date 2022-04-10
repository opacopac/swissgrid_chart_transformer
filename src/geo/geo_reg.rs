use std::fmt;
use crate::{Ch1903Coord, Position2d};
use crate::geo::coord::Coord;


pub struct GeoReg {
    pub x_coord_per_px_width: f32,
    pub y_coord_per_px_width: f32,
    pub x_coord_per_px_height: f32,
    pub y_coord_per_px_height: f32,
    pub coord_tl: (f32, f32),
    det: f32,
}


impl fmt::Display for GeoReg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x/px width: {}, y/px width: {}, x/px height: {}, y/px height: {}\ncoord tl: {}, {}",
            self.x_coord_per_px_width,
            self.y_coord_per_px_width,
            self.x_coord_per_px_height,
            self.y_coord_per_px_height,
            self.coord_tl.0,
            self.coord_tl.1
        )
    }
}


impl GeoReg {
    const MM_PER_INCH: f32 = 25.4; // TODO


    pub fn new(
        x_coord_per_px_width: f32,
        y_coord_per_px_width: f32,
        x_coord_per_px_height: f32,
        y_coord_per_px_height: f32,
        coord_tl: (f32, f32),
    ) -> GeoReg {
        let det = 1.0 / (x_coord_per_px_width * y_coord_per_px_height - y_coord_per_px_width * x_coord_per_px_height);

        return GeoReg {
            x_coord_per_px_width,
            y_coord_per_px_width,
            x_coord_per_px_height,
            y_coord_per_px_height,
            coord_tl,
            det
        };
    }


    pub fn from_pos1_size_scale(
        px1: (f32, f32),
        coord1: (f32, f32),
        img_size_px: (f32, f32),
        chart_scale: f32,
        resolution_dpi: f32
    ) -> GeoReg {
        let mm_per_inch = 25.4;
        let width_mm = img_size_px.0 / resolution_dpi * mm_per_inch;
        let height_mm = img_size_px.1 / resolution_dpi * mm_per_inch;
        let x_coord_per_px = width_mm / img_size_px.0 / 1000.0 * chart_scale;
        let y_coord_per_px = -height_mm / img_size_px.1 / 1000.0 * chart_scale;
        let x_coord_tl = coord1.0 - px1.0 * x_coord_per_px;
        let y_coord_tl = coord1.1 - px1.1 * y_coord_per_px;

        return GeoReg::new(
            x_coord_per_px,
            0.0,
            0.0,
            y_coord_per_px,
            (x_coord_tl, y_coord_tl)
        );
    }


    pub fn from_pos1_pos2_stretch(
        px1: (f32, f32),
        coord1: (f32, f32),
        px2: (f32, f32),
        coord2: (f32, f32),
    ) -> GeoReg {
        let px_diff_x = px2.0 - px1.0;
        let px_diff_y = px2.1 - px1.1;
        let coord_diff_x = coord2.0 - coord1.0;
        let coord_diff_y = coord2.1 - coord1.1;
        let x_coord_per_px = coord_diff_x / px_diff_x;
        let y_coord_per_px = coord_diff_y / px_diff_y;
        let x_coord_tl = coord1.0 - px1.0 * x_coord_per_px;
        let y_coord_tl = coord1.1 - px1.1 * y_coord_per_px;

        return GeoReg::new(
            x_coord_per_px,
            0.0,
            0.0,
            y_coord_per_px,
            (x_coord_tl, y_coord_tl)
        );
    }


    pub fn from_pos1_pos2_rot(
        px1: (f32, f32),
        coord1: (f32, f32),
        px2: (f32, f32),
        coord2: (f32, f32),
    ) -> GeoReg {
        let px_diff_x = px2.0 - px1.0;
        let px_diff_y = px2.1 - px1.1;
        let px_diff = (px_diff_x * px_diff_x + px_diff_y * px_diff_y).sqrt();
        let px_rot_rad = px_diff_y.atan2(px_diff_x);

        let coord_diff_x = coord2.0 - coord1.0;
        let coord_diff_y = coord2.1 - coord1.1;
        let coord_diff = (coord_diff_x * coord_diff_x + coord_diff_y * coord_diff_y).sqrt();
        let coord_rot_rad = -coord_diff_y.atan2(coord_diff_x);
        let coord_per_pixel = coord_diff / px_diff;
        let rot_rad = px_rot_rad - coord_rot_rad;

        let x_coord_per_px_width = coord_per_pixel * rot_rad.cos();
        let y_coord_per_px_width = coord_per_pixel * rot_rad.sin();
        let x_coord_per_px_height = y_coord_per_px_width;
        let y_coord_per_px_height = -x_coord_per_px_width;
        let x_coord_tl = coord1.0 - px1.0 * x_coord_per_px_width - px1.1 * x_coord_per_px_height;
        let y_coord_tl = coord1.1 - px1.0 * y_coord_per_px_width - px1.1 * y_coord_per_px_height;

        return GeoReg::new(
            x_coord_per_px_width,
            y_coord_per_px_width,
            x_coord_per_px_height,
            y_coord_per_px_height,
            (x_coord_tl, y_coord_tl)
        );
    }


    pub fn calc_coord_by_px(&self, px_x: f32, px_y: f32) -> (f32, f32) {
        let coord_x = self.x_coord_per_px_width * px_x + self.x_coord_per_px_height * px_y + self.coord_tl.0;
        let coord_y = self.y_coord_per_px_width * px_x + self.y_coord_per_px_height * px_y + self.coord_tl.1;

        return (coord_x, coord_y);
    }


    pub fn calc_px_by_coord(&self, coord: (f32, f32)) -> (f32, f32) {
        let x_rel = coord.0 - self.coord_tl.0;
        let y_rel = coord.1 - self.coord_tl.1;
        let px_x = self.det * (self.y_coord_per_px_height * x_rel - self.x_coord_per_px_height * y_rel);
        let px_y = self.det * (-self.y_coord_per_px_width * x_rel + self.x_coord_per_px_width * y_rel);

        return (px_x, px_y);
    }
}
