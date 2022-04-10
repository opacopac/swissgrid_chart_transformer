use std::fs;
use crate::geo::geo_reg::GeoReg;

pub struct WorldFileService;


impl WorldFileService {
    pub fn read(filename: &str) -> GeoReg {
        let file_content = fs::read_to_string(filename).unwrap(); // TODO

        return WorldFileService::parse_file_content(file_content);
    }


    pub fn save(geo_reg: GeoReg, filename: &str) {
        let content = WorldFileService::create_file_content(geo_reg);

        fs::write(filename, content); // TODO
    }


    fn parse_file_content(file_conent: String) -> GeoReg {
        let mut lines = file_conent.lines();
        let x_coord_per_px_width = lines.next().unwrap().parse::<f32>().unwrap(); // TODO
        let y_coord_per_px_width = lines.next().unwrap().parse::<f32>().unwrap();
        let x_coord_per_px_height = lines.next().unwrap().parse::<f32>().unwrap();
        let y_coord_per_px_height = lines.next().unwrap().parse::<f32>().unwrap();
        let x_coord_tl = lines.next().unwrap().parse::<f32>().unwrap();
        let y_coord_tl = lines.next().unwrap().parse::<f32>().unwrap();

        return GeoReg::new(
            x_coord_per_px_width,
            y_coord_per_px_width,
            x_coord_per_px_height,
            y_coord_per_px_height,
            (x_coord_tl, y_coord_tl)
        );
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
