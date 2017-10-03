include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl Circle {
    pub fn area(&self) -> f32 {
        unsafe { circle_area(self) }
    }
}

impl Point {
    pub fn info(&self) -> String {
        use std::ffi::CString;

        unsafe {
            let string = CString::new("").unwrap();
            let pointer = string.into_raw();
            point_info(self, pointer);

            CString::from_raw(pointer).into_string().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_area() {
        let radius: f32 = 3.;
        let point = build_point();
        let circle = Circle { center: &point, radius: radius };

        let area = radius.powi(2) * ::std::f32::consts::PI;
        assert_eq!(area, circle.area());
    }

    #[test]
    fn point_info() {
        let point = build_point();
        let info = "x: 3.000000,\ny: 4.000000";

        assert_eq!(info, point.info());
    }

    fn build_point() -> Point {
        Point { x: 3., y: 4. }
    }
}
