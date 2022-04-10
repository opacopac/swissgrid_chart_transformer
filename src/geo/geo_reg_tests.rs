#[cfg(test)]
mod geo_reg_tests {
    use assert_approx_eq::assert_approx_eq;
    use crate::geo::geo_reg::GeoReg;
    use crate::Position2d;

    #[test]
    fn it_creates_an_instance() {
        let coord_tl = (7.0, 47.0);

        let result = GeoReg::new(0.1, 0.2, 0.3, 0.4, coord_tl);

        assert_eq!(result.x_coord_per_px_width, 0.1);
        assert_eq!(result.y_coord_per_px_width, 0.2);
        assert_eq!(result.x_coord_per_px_height, 0.3);
        assert_eq!(result.y_coord_per_px_height, 0.4);
        assert_eq!(result.coord_tl.0, 7.0);
        assert_eq!(result.coord_tl.1, 47.0);
    }


    #[test]
    fn it_calculates_the_top_left_coordinate() {
        let coord_tl = (7.0, 47.0);
        let geo_reg = GeoReg::new(0.1, 0.1, 0.1, -0.1, coord_tl);

        let result = geo_reg.calc_coord_by_px(0.0, 0.0);

        assert_eq!(result.0, 7.0);
        assert_eq!(result.1, 47.0);
    }


    #[test]
    fn it_calculates_the_coordinate_from_a_pixel_no_rot() {
        let coord_tl = (7.0, 47.0);
        let geo_reg = GeoReg::new(0.1, 0.0, 0.0, -0.2, coord_tl);

        let result = geo_reg.calc_coord_by_px(3.0, 4.0);

        assert_eq!(result.0, 7.3);
        assert_eq!(result.1, 46.2);
    }


    #[test]
    fn it_calculates_the_coordinate_from_a_pixel_rot_90() {
        let coord_tl = (7.0, 47.0);
        let geo_reg = GeoReg::new(0.0, -0.1, 0.2, 0.0, coord_tl);

        let result = geo_reg.calc_coord_by_px(3.0, 4.0);

        assert_eq!(result.0, 7.8);
        assert_eq!(result.1, 46.7);
    }


    #[test]
    fn it_calculates_the_pixel_from_the_top_left_coordinate() {
        let coord_tl = (7.0, 47.0);
        let geo_reg = GeoReg::new(0.1, 0.0, 0.0, -0.2, coord_tl);

        let result = geo_reg.calc_px_by_coord(coord_tl);

        assert_eq!(result.0, 0.0);
        assert_eq!(result.1, 0.0);
    }


    #[test]
    fn it_calculates_the_pixel_from_a_coordinate_no_rot() {
        let coord_tl = (7.0, 47.0);
        let geo_reg = GeoReg::new(0.1, 0.0, 0.0, -0.2, coord_tl);
        let coord = (7.3, 46.2);

        let result = geo_reg.calc_px_by_coord(coord);

        assert_approx_eq!(result.0, 3.0, 1e-5);
        assert_approx_eq!(result.1, 4.0, 1e-5);
    }


    #[test]
    fn it_creates_from_pos1_pos2_stretch() {
        let px1 = (10.0, 110.0);
        let coord1 = (7.0, 47.0);
        let px2 = (110.0, 10.0);
        let coord2 = (8.0, 48.0);

        let result = GeoReg::from_pos1_pos2_stretch(px1, coord1, px2, coord2);

        assert_eq!(result.x_coord_per_px_width, 0.01);
        assert_eq!(result.y_coord_per_px_width, 0.0);
        assert_eq!(result.x_coord_per_px_height, 0.0);
        assert_eq!(result.y_coord_per_px_height, -0.01);
        assert_eq!(result.coord_tl.0, 6.9);
        assert_eq!(result.coord_tl.1, 48.1);
    }


    #[test]
    fn it_creates_from_pos1_pos2_rot() {
        let px1 = (0.0, 0.0);
        let coord1 = (7.0, 47.0);
        let px2 = (45.0, 45.0);
        let coord2 = (7.5, 46.5);

        let result = GeoReg::from_pos1_pos2_rot(px1, coord1, px2, coord2);

        println!("{}", result);

        assert_eq!(result.x_coord_per_px_width, 0.01);
        assert_eq!(result.y_coord_per_px_width, 0.0);
        assert_eq!(result.x_coord_per_px_height, 0.0);
        assert_eq!(result.y_coord_per_px_height, -0.01);
        assert_eq!(result.coord_tl.0, 6.9);
        assert_eq!(result.coord_tl.1, 48.1);
    }

}
