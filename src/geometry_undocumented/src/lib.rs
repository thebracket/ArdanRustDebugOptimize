//#![warn(missing_docs)]
use std::f64::consts::PI;

pub fn area_of_a_circle(radius: f64) -> f64 {
    PI * radius * radius
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn circle_area() {
        assert_eq!(format!("{:.4}", area_of_a_circle(10.0)), "314.1593");
    }
}