use std::error::Error;
use std::fs;
use std::path::Path;

use simple_error::bail;

use crate::geo::geo_reg::GeoReg;

pub struct WorldFileService;


impl WorldFileService {
    pub fn read(filename: &str) -> Result<GeoReg, Box<dyn Error>> {
        let file_content = fs::read_to_string(filename)?;
        let geo_reg = WorldFileService::parse_file_content(file_content)?;

        return Ok(geo_reg);
    }


    pub fn save(geo_reg: GeoReg, filename: &str) {
        let content = WorldFileService::create_file_content(geo_reg);
        let _result = fs::write(filename, content);
    }


    pub fn derive_file_name(image_file: &str) -> Result<String, Box<dyn Error>> {
        let path = Path::new(image_file);
        let new_ext = match path.extension().and_then(|x| x.to_str()) {
            Some(ext) => format!("{}w", ext),
            None => "wld".to_string(),
        };
        let world_file_name = path.with_extension(new_ext)
            .to_str()
            .ok_or("Invalid UTF-8 in path")?
            .to_string();

        Ok(world_file_name)
    }


    fn parse_file_content(file_conent: String) -> Result<GeoReg, Box<dyn Error>> {
        let lines: Vec<_> = file_conent.lines().collect();

        if lines.len() < 6 {
            bail!("invalid world file format");
        }

        let x_coord_per_px_width = lines[0].parse::<f32>()?;
        let y_coord_per_px_width = lines[1].parse::<f32>()?;
        let x_coord_per_px_height = lines[2].parse::<f32>()?;
        let y_coord_per_px_height = lines[3].parse::<f32>()?;
        let x_coord_tl = lines[4].parse::<f32>()?;
        let y_coord_tl = lines[5].parse::<f32>()?;
        
        let geo_reg = GeoReg::new(
            x_coord_per_px_width,
            y_coord_per_px_width,
            x_coord_per_px_height,
            y_coord_per_px_height,
            (x_coord_tl, y_coord_tl)
        );
        
        return Ok(geo_reg);
    }


    fn create_file_content(geo_reg: GeoReg) -> String {
        return format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n",
            geo_reg.x_coord_per_px_width,
            geo_reg.y_coord_per_px_width,
            geo_reg.x_coord_per_px_height,
            geo_reg.y_coord_per_px_height,
            geo_reg.coord_tl.0,
            geo_reg.coord_tl.1
        );
    }
}
