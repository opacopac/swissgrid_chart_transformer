use image::ColorType;
use crate::Image;
use crate::imaging::error::ImagingError;
use crate::imaging::NUM_COLOR_VALUES;


pub struct Drawable {
    color_values: Vec<u8>,
    width: u32,
    height: u32
}


impl Drawable {
    const INIT_COLOR_VALUE: u8 = 50;


    pub fn create(width: u32, height: u32) -> Result<Drawable, ImagingError> {
        if width == 0 || height == 0 {
            return Err(ImagingError::InvalidArgumentError(String::from("width/height must not be 0")));
        }

        let px_count = (width * height * NUM_COLOR_VALUES) as usize;
        let mut color_values = Vec::new();
        color_values.resize(px_count, Drawable::INIT_COLOR_VALUE);

        return Result::Ok(Drawable { color_values, width, height });
    }


    pub fn draw_point(&mut self, x: u32, y: u32, color: [u8; 4]) {
        if x >= self.width || y >= self.height {
            panic!("coordinates out of bound");
        }

        let idx = ((y * self.width + x) * NUM_COLOR_VALUES) as usize;

        self.color_values[idx] = color[0];
        self.color_values[idx + 1] = color[1];
        self.color_values[idx + 2] = color[2];
        self.color_values[idx + 3] = color[3];
    }


    pub fn safe_image(&self, filename: &str) -> Result<bool, ImagingError> {
        image::save_buffer(
            filename,
            &*self.color_values,
            self.width,
            self.height,
            ColorType::Rgba8
        )?;

        return Result::Ok(true);
    }
}
