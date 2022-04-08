use std::fmt;
use std::sync::Mutex;

use min_max::{max, max_partial, min, min_partial};
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

use crate::{Ch1903Coord, Drawable, Image, Position2d};
use crate::geo::extent_2d::Extent2d;

pub struct Ch1903Chart {
    image: Image,
    pixel_pos_1: (f32, f32),
    ch_1903_coord: Ch1903Coord,
    x_coord_per_pixel: f32,
    y_coord_per_pixel: f32,
    rotation_rad: f32
}


impl fmt::Display for Ch1903Chart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pixel_pos_1: {}, coord: {}:{}, xpp: {}, ypp: {}, rot: {}",
            self.pixel_pos_1.0,
            self.pixel_pos_1.1,
            self.ch_1903_coord,
            self.x_coord_per_pixel,
            self.y_coord_per_pixel,
            self.rotation_rad
        )
    }
}


impl Ch1903Chart {
    const MM_PER_INCH: f32 = 25.4; // TODO


    pub fn from_pos1_and_pos2(
        image: Image,
        pixel_pos1: (u32, u32),
        ch_coordinate1: Ch1903Coord,
        pixel_pos2: (u32, u32),
        ch_coordinate2: Ch1903Coord,
    ) -> Ch1903Chart {
        let px_diff_x = pixel_pos2.0 as f32 - pixel_pos1.0 as f32;
        let px_diff_y = pixel_pos2.1 as f32 - pixel_pos1.1 as f32;
        let coord_diff_e = ch_coordinate2.e - ch_coordinate1.e;
        let coord_diff_n = ch_coordinate2.n - ch_coordinate1.n;
        let x_coord_per_pixel = coord_diff_e / px_diff_x;
        let y_coord_per_pixel = coord_diff_n / px_diff_y;

        println!("px pos 1 x/y: {} {}", pixel_pos1.0, pixel_pos1.1);
        println!("coord 1 e/n: {} {}", ch_coordinate1.e, ch_coordinate1.n);
        println!("px diff x/y: {} {}", px_diff_x, px_diff_y);
        println!("coord diff e/n: {} {}", coord_diff_e, coord_diff_n);
        println!("coord per pixel x/y: {} {}", x_coord_per_pixel, y_coord_per_pixel);

        return Ch1903Chart {
            image,
            pixel_pos_1: (pixel_pos1.0 as f32, pixel_pos1.1 as f32),
            ch_1903_coord: ch_coordinate1,
            x_coord_per_pixel,
            y_coord_per_pixel,
            rotation_rad: 0.0
        }
    }


    pub fn from_pos_and_scale(
        image: Image,
        pixel_pos1: (u32, u32),
        ch_coordinate1: Ch1903Coord,
        chart_scale: f32,
        resolution_dpi: f32
    ) -> Ch1903Chart {
        let img_width = image.width() as f32;
        let img_height = image.height() as f32;
        let width_mm = img_width / resolution_dpi * Ch1903Chart::MM_PER_INCH;
        let height_mm = img_height as f32 / resolution_dpi * Ch1903Chart::MM_PER_INCH;

        return Ch1903Chart {
            image,
            pixel_pos_1: (pixel_pos1.0 as f32, pixel_pos1.1 as f32),
            ch_1903_coord: ch_coordinate1,
            x_coord_per_pixel: width_mm / img_width / 1000.0 * chart_scale,
            y_coord_per_pixel: -height_mm / img_height / 1000.0 * chart_scale,
            rotation_rad: 0.0
        }
    }


    pub fn from_pos1_pos2_rot(
        image: Image,
        pixel_pos1: (u32, u32),
        ch_coordinate1: Ch1903Coord,
        pixel_pos2: (u32, u32),
        ch_coordinate2: Ch1903Coord,
    ) -> Ch1903Chart {
        let px_diff_x = (pixel_pos2.0 - pixel_pos1.0) as f32;
        let px_diff_y = (pixel_pos2.1 - pixel_pos1.1) as f32;
        let px_diff = (px_diff_x * px_diff_x + px_diff_y * px_diff_y).sqrt();
        let px_rot_rad = px_diff_y.atan2(px_diff_x);

        let coord_diff_e = ch_coordinate2.e - ch_coordinate1.e;
        let coord_diff_n = ch_coordinate2.n - ch_coordinate1.n;
        let coord_diff = (coord_diff_e * coord_diff_e + coord_diff_n * coord_diff_n).sqrt();
        let coord_rot_rad = -coord_diff_n.atan2(coord_diff_e);

        let coord_per_pixel = coord_diff / px_diff;
        let rot_rad = px_rot_rad - coord_rot_rad;

        return Ch1903Chart {
            image,
            pixel_pos_1: (pixel_pos1.0 as f32, pixel_pos1.1 as f32),
            ch_1903_coord: ch_coordinate1,
            x_coord_per_pixel: coord_per_pixel,
            y_coord_per_pixel: -coord_per_pixel,
            rotation_rad: rot_rad
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
        let mut px_rel_x = x as f32 - self.pixel_pos_1.0;
        let mut px_rel_y = y as f32 - self.pixel_pos_1.1;

        if self.rotation_rad != 0.0 {
            let neg_rot = -self.rotation_rad;
            let px_rel_x2 = px_rel_x * neg_rot.cos() - px_rel_y * neg_rot.sin();
            px_rel_y = px_rel_x * neg_rot.sin() + px_rel_y * neg_rot.cos();
            px_rel_x = px_rel_x2;
        }

        let ch_coord_e = px_rel_x * self.x_coord_per_pixel + self.ch_1903_coord.e;
        let ch_coord_n = px_rel_y * self.y_coord_per_pixel + self.ch_1903_coord.n;

        return Ch1903Coord { e: ch_coord_e, n: ch_coord_n };
    }


    pub fn get_pixel_color(&self, ch_coord: Ch1903Coord) -> [u8; 4] {
        let mut px_rel_x = (ch_coord.e - self.ch_1903_coord.e) / self.x_coord_per_pixel;
        let mut px_rel_y = (ch_coord.n - self.ch_1903_coord.n) / self.y_coord_per_pixel;

        if self.rotation_rad != 0.0 {
            let px_rel_x2 = px_rel_x * self.rotation_rad.cos() - px_rel_y * self.rotation_rad.sin();
            px_rel_y = px_rel_x * self.rotation_rad.sin() + px_rel_y * self.rotation_rad.cos();
            px_rel_x = px_rel_x2;
        }

        px_rel_x += self.pixel_pos_1.0 as f32;
        px_rel_y += self.pixel_pos_1.1 as f32;

        return self.image.interpolate_pixel_color(px_rel_x, px_rel_y);
    }
}
