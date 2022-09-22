
//extern crate vehicle;
//use vehicle::formulas::{Point, SlopePoints, slope_from_points};

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct SlopePoints {
    pub start: Point,
    pub end: Point,
}

/// Slope Formula (m = (y2 - y1)/(x2 - x1) = Δy/Δx)
/// 
/// example:
/// 
/// ```
/// use::vehicle::formulas::{Point, SlopePoints, slope_from_points};
/// 
/// let points = SlopePoints {
///     start: Point { x: 0.0, y: 0.0 },
///     end: Point { x: 10.0, y: 10.0 },
/// };
/// let slope = slope_from_points(&points);
/// assert_eq!(slope.x, 10.0);
/// assert_eq!(slope.y, 10.0);
/// assert_eq!(slope.y/slope.x, 1.0);
/// println!("m = Δ{}/Δ{}", slope.y, slope.x);
/// ```
pub fn slope_from_points(points: &SlopePoints) -> Point {
    Point {
        x: (points.end.x - points.start.x),
        y: (points.end.y - points.start.y)
    }
}



#[derive(Debug)]
pub struct Slope {
    pub slope: SlopePoints,
}

impl Slope {
    pub fn get_range_from_interval(&self, interval: f64) -> Vec<Point> {
        /*
        let slope = slope_from_points(&self.slope);

        let count = self.slope.end.x / interval;
        let points = Vec::new();
        for i in 1..count {
            let value = i*interval;
            if value >= self.slope.end.x {
                break
            }
        }
        */
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope_from_points() {
        let points = SlopePoints {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 10.0, y: 10.0 },
        };
        let slope = slope_from_points(&points);
        assert_eq!(slope.x, 10.0);
        assert_eq!(slope.y, 10.0);
        assert_eq!(slope.y/slope.x, 1.0);
        println!("m = Δ{}/Δ{}", slope.y, slope.x);
    }
}