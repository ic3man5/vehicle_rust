//! Calculations useful involving Tires
use regex::Regex;

use crate::formulas;

pub struct Tire {
    /// Diameter of the tire in inches
    pub diameter: f64,
}

impl Tire {
    /// Create a new Tire based on the metric string of the tire.
    ///
    /// example:
    ///
    /// ```
    /// use vehicle::tire::Tire;
    ///
    /// let tire = Tire::new("275/55R20");
    ///
    /// println!("{}\" diameter {}\" Circumference", tire.diameter, tire.circumference());
    /// ```
    pub fn new(value: &str) -> Tire {
        let re = Regex::new(r"\d{2,}").unwrap();
        assert!(re.is_match(value));

        let results: Vec<f64> = re
            .find_iter(&value)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();
        let width = results.get(0).unwrap();
        let aspect_ratio = results.get(1).unwrap();
        let wheel_diameter = results.get(2).unwrap();

        let height_mm = (width * (aspect_ratio / 100.0)) * 2.0;
        let diameter = formulas::to_in(height_mm / 10.0) + wheel_diameter;

        Tire { diameter }
    }

    /// Returns the circumference of the Tire based on the diameter.
    ///
    /// example:
    ///
    /// ```
    /// use vehicle::tire::Tire;
    ///
    /// let tire = Tire::new("275/55R20");
    ///
    /// println!("{:2}\" Circumference", tire.circumference());
    /// ```
    pub fn circumference(&self) -> f64 {
        self.diameter * 3.14
    }

    /// Returns the distance in miles per tire revolution.
    ///
    /// example:
    ///
    /// ```
    /// use vehicle::tire::Tire;
    ///
    /// let tire = Tire::new("275/55R20");
    ///
    /// println!("{:2} mile(s) per revolution", tire.miles_per_rev());
    /// ```
    pub fn miles_per_rev(&self) -> f64 {
        self.circumference() / (5280.0 * 12.0)
    }

    /// Returns the revolutions per mile of the tire.
    ///
    /// example:
    ///
    /// ```
    /// use vehicle::tire::Tire;
    ///
    /// let tire = Tire::new("275/55R20");
    ///
    /// println!("{:2} revolution(s) per mile", tire.revs_per_mile());
    /// ```
    pub fn revs_per_mile(&self) -> f64 {
        1.0 / self.miles_per_rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tire() {
        let tire = Tire::new("275/55R20");
        println!(
            "{}\" diameter {}\" Circumference",
            tire.diameter,
            tire.circumference()
        );
        println!(
            "{}\" miles per rev {}\" revs per mile",
            tire.miles_per_rev(),
            tire.revs_per_mile()
        );
        assert_eq!(tire.diameter, 31.909448818897637);
        assert_eq!(tire.circumference(), 100.19566929133859);
        assert_eq!(tire.miles_per_rev(), 0.001581371043108248);
        assert_eq!(tire.revs_per_mile(), 632.362660463581);
    }
}
