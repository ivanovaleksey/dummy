#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use Circle;
    use Point;

    use circle_area;

    #[test]
    fn calculate_circle_area() {
        let radius: f32 = 3.;
        let point = Point { x: 3., y: 4. };
        let circle = Circle { center: &point, radius: radius };

        let area = radius.powi(2) * ::std::f32::consts::PI;
        assert_eq!(area, unsafe { circle_area(&circle) });
    }
}
