use std::error::Error;

use crate::{Ch1903GeoRegChart, Drawable};
use crate::chart::common_chart_projection_service::CommonChartProjectionService;
use crate::chart::projection_result::ProjectionResult;
use crate::geo::geo_reg::GeoReg;

pub struct SingleChartProjectionService;


impl SingleChartProjectionService {
    pub fn create_chart(chart: &Ch1903GeoRegChart) -> Result<ProjectionResult, Box<dyn Error>> {
        let extent = CommonChartProjectionService::calc_lat_lon_extent(chart);
        let mid_pos = extent.calc_mid_pos();
        let lon_diff = extent.max_pos.lon - extent.min_pos.lon;
        let lat_diff = extent.max_pos.lat - extent.min_pos.lat;
        let px_width = chart.width();
        let px_per_deg = px_width as f32 / lon_diff;
        let lat_rad = mid_pos.lat.to_radians();
        let px_height = (lat_diff * px_per_deg / lat_rad.cos()).round() as u32;
        let lon_inc = lon_diff / (px_width as f32 - 1.0);
        let lat_inc = lat_diff / (px_height as f32 - 1.0);

        let px_rows = CommonChartProjectionService::calc_area_projection(
            &chart,
            px_width,
            px_height,
            extent.min_pos.lon,
            extent.min_pos.lat,
            lon_inc,
            lat_inc
        );
        let drawable = Drawable::create_with_data(px_width, px_height, px_rows)?;
        let geo_reg = GeoReg::new(
            lon_inc,
            lat_inc,
            0.0,
            0.0,
            (extent.min_pos.lon, extent.max_pos.lat)
        );

        return Ok(ProjectionResult::new(drawable, geo_reg));
    }
}
