use crate::geo::geo_reg::GeoReg;
use crate::imaging::drawable::Drawable;


pub struct ProjectionResult {
    pub drawable: Drawable,
    pub geo_reg: GeoReg
}


impl ProjectionResult {
    pub fn new(drawable: Drawable, geo_reg: GeoReg) -> ProjectionResult {
        return ProjectionResult { drawable, geo_reg };
    }
}
