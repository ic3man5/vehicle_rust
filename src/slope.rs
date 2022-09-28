//! Used for calculating the slope of a line

#[derive(Debug)]
/// Represents a point on an X, Y plane/graph.
/// 
/// example:
/// 
/// ```
/// use vehicle::slope::Point;
/// 
/// let point = Point::from(1.0, 3.2);
/// assert_eq!(point.x, 1.0);
/// assert_eq!(point.y, 3.2);
/// ```
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Zero initializes a Point struct
    pub fn new() -> Self {
        Self { 
            x: 0.0, 
            y: 0.0,
        }
    }

    pub fn from(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
        }
    }
}

#[derive(Debug)]
/// Slope Formula (m = (y2 - y1)/(x2 - x1) = Δy/Δx)
/// 
/// example:
/// 
/// ```
/// use vehicle::slope::{Point, Slope};
/// 
/// let slope = Slope::from(0.0, 0.0, 2.0, 2.0);
/// let (m, p) = slope.m();
/// assert_eq!(p.x, 2.0);
/// assert_eq!(p.y, 2.0);
/// assert_eq!(p.y/p.x, 1.0);
/// assert_eq!(m, 1.0);
/// println!("{} = Δ{}/Δ{}", m, p.y, p.x);
/// ```
pub struct Slope {
    pub start: Point,
    pub end: Point,
}

impl Slope {

    pub fn from(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            start: Point {
                x: x1,
                y: y1,
            },
            end: Point {
                x: x2,
                y: y2,
            },
        }
    }

    pub fn from_points(start: Point, end: Point) -> Self {
        Self {
            start,
            end,
        }
    }
    /// Calculate the slope (m = (y2 - y1)/(x2 - x1) = Δy/Δx)
    /// 
    /// example:
    /// 
    /// ```
    /// use::vehicle::slope::{Point, Slope};
    /// 
    /// let slope = Slope {
    ///     start: Point::from(0.0, 0.0),
    ///     end: Point::from(10.0, 10.0),
    /// };
    /// let (m, p) = slope.m();
    /// assert_eq!(p.x, 10.0);
    /// assert_eq!(p.y, 10.0);
    /// assert_eq!(p.y/p.x, 1.0);
    /// assert_eq!(m, 1.0);
    /// println!("{} = Δ{}/Δ{}", m, p.y, p.x);
    /// ```
    pub fn m(&self) -> (f64, Point) {
        // Calculate the slope points
        let point = Point {
            x: (self.end.x - self.start.x),
            y: (self.end.y - self.start.y)
        };
        // Make sure we don't divide by zero
        let m = if point.x == 0.0 {
            0.0
        } else {
            point.y / point.x
        };
        (m, point)
    }
}
