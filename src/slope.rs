
//extern crate vehicle;
//use vehicle::formulas::{Point, SlopePoints, slope_from_points};

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn from(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }

    /// Calculates the actual slope (m) from x and y
    pub fn slope(&self) -> f64 {
        self.y / self.x
    }
}

#[derive(Debug)]
pub struct SlopePoints {
    pub start: Point,
    pub end: Point,
}

impl SlopePoints {
    /// Slope Formula (m = (y2 - y1)/(x2 - x1) = Δy/Δx)
    /// 
    /// example:
    /// 
    /// ```
    /// use::vehicle::slope::{Point, SlopePoints, slope_from_points};
    /// 
    /// let points = SlopePoints {
    ///     start: Point { x: 0.0, y: 0.0 },
    ///     end: Point { x: 10.0, y: 10.0 },
    /// };
    /// let slope = points.slope();
    /// assert_eq!(slope.x, 10.0);
    /// assert_eq!(slope.y, 10.0);
    /// assert_eq!(slope.y/slope.x, 1.0);
    /// println!("m = Δ{}/Δ{}", slope.y, slope.x);
    /// ```
    pub fn slope(&self) -> Point {
        Point {
            x: (self.end.x - self.start.x),
            y: (self.end.y - self.start.y)
        }
    }
    
    pub fn m(&self) -> f64 {
        let slope = self.slope();
        slope.y / slope.x
    }
    
}

#[derive(Debug)]
pub struct Slope {
    pub points: SlopePoints,
}

impl Slope {
    /// Gets the range of points required to calculate the linear line on a graph
    /// interval is the interval of the x axis.
    /// 
    /// example:
    /// 
    /// ```
    /// use::vehicle::slope::{Point, SlopePoints, Slope};
    /// 
    /// let points = SlopePoints {
    ///     start: Point { x: 0.0, y: 0.0 },
    ///     end: Point { x: 10.0, y: 10.0 },
    /// };
    /// let slope = Slope { points };
    /// let slope_range = slope.get_range_from_interval(1.0);
    /// println!("{:#?}", slope_range);
    /// ```
    pub fn get_range_from_interval(&self, interval: f64) -> Vec<Point> {
        let m = self.points.slope();
        //let m = slope.y / slope.x;
        let mut points = Vec::new();
        let mut i = 0.0_f64;
        loop {
            let value = i*interval;
            if value > self.points.end.x {
                break;
            }
            points.push(Point::from(i, m*i));
            i += interval;
        }
        points
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

    #[test]
    fn test_slope_get_range_from_interval() {
        let points = SlopePoints {
            start: Point { x: 0.0, y: 0.0 },
            end: Point { x: 100.0, y: 100.0 },
        };
        let slope = Slope { points };
        let slope_range = slope.get_range_from_interval(10.0);
        println!("{:#?}", slope_range);
        for i in 0..10 {
            let x = i as f64 * 10.0;
            assert_eq!(x, slope_range[i].x);
            assert_eq!(x, slope_range[i].y);
        }
    }
}