use std::error::Error;
use image::{EncodableLayout, ImageResult, io};
use crate::imaging::{COLOR_TRANSPARENT, NUM_COLOR_VALUES};
use crate::imaging::error::ImagingError;



pub struct Image {
    pixel_values: Vec<u8>,
    width: u32,
    height: u32
}


impl Image {
    pub fn load_img(filename: &str) -> Result<Image, ImagingError> {
        let img = image::open(filename)?;
        let width = img.width();
        let height = img.height();
        let pixel_values = img.into_rgba8().into_vec();

        return Result::Ok(Image { pixel_values, width, height });
    }


    pub fn width(&self) -> u32 {
        return self.width;
    }


    pub fn height(&self) -> u32 {
        return self.height;
    }


    pub fn get_pixel_color(&self, x: u32, y: u32) -> [u8; 4] {
        if x >= self.width || y >= self.height {
            return COLOR_TRANSPARENT;
        }

        let idx = ((y * self.width + x) * NUM_COLOR_VALUES) as usize;

        return [
            self.pixel_values[idx],
            self.pixel_values[idx + 1],
            self.pixel_values[idx + 2],
            self.pixel_values[idx + 3]
        ];
    }


    pub fn interpolate_pixel_color(&self, x: f32, y: f32) -> [u8; 4] {
        let floor_x_f32 = x.floor();
        let floor_y_f32 = y.floor();
        let ceil_x_f32 = x.ceil();
        let ceil_y_f32 = y.ceil();

        if ceil_x_f32 < 0.0 || ceil_y_f32 < 0.0 || floor_x_f32 >= self.width as f32 || floor_y_f32 >= self.height as f32 {
            return COLOR_TRANSPARENT;
        }

        let floor_x = floor_x_f32 as u32;
        let floor_y = floor_y_f32 as u32;
        let ceil_x = ceil_x_f32 as u32;
        let ceil_y = ceil_y_f32 as u32;

        if floor_x == ceil_x || floor_y == ceil_y {
            return self.get_pixel_color(floor_x, floor_y);
        }

        let col_tl = self.get_pixel_color(floor_x, floor_y);
        let col_tr = self.get_pixel_color(ceil_x, floor_y);
        let col_bl = self.get_pixel_color(floor_x, ceil_y);
        let col_br = self.get_pixel_color(ceil_x, ceil_y);
        let col_t = Image::interpolate_color(col_tl, ceil_x_f32 - x, col_tr, x - floor_x_f32);
        let col_b = Image::interpolate_color(col_bl, ceil_x_f32 - x, col_br, x - floor_x_f32);
        let col = Image::interpolate_color(col_t, ceil_y_f32 - y, col_b, y - floor_y_f32);

        return col;
    }


    fn interpolate_color(color1: [u8; 4], weight1: f32, color2: [u8; 4], weight2: f32) -> [u8; 4] {
        return [
            (color1[0] as f32 * weight1 + color2[0] as f32 * weight2) as u8,
            (color1[1] as f32 * weight1 + color2[1] as f32 * weight2) as u8,
            (color1[2] as f32 * weight1 + color2[2] as f32 * weight2) as u8,
            (color1[3] as f32 * weight1 + color2[3] as f32 * weight2) as u8,
        ];
    }
}
