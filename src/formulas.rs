//! Formulas useful for vehicles

/// Converts inches to centimeters
///
/// example:
///
/// ```
/// use vehicle::formulas::to_cm;
///
/// let inches = 1.0;
/// let cm = to_cm(inches);
/// ```
pub fn to_cm(inches: f64) -> f64 {
    inches * 2.54
}

/// Converts centimeters to inches
///
/// example:
///
/// ```
/// use vehicle::formulas::to_in;
///
/// let centimeters = 25.4;
/// let inches = to_in(centimeters);
/// ```
pub fn to_in(centimeters: f64) -> f64 {
    centimeters / 2.54
}

/// Converts mph to kph
///
/// example:
///
/// ```
/// use vehicle::formulas::to_kph;
///
/// let mph = 100.0;
/// let kph = to_kph(mph);
/// ```
pub fn to_kph(mph: f64) -> f64 {
    mph * 1.609344
}

/// Converts kph to mph
///
/// example:
///
/// ```
/// use vehicle::formulas::to_mph;
///
/// let kph = 100.0;
/// let mph = to_mph(kph);
/// ```
pub fn to_mph(kph: f64) -> f64 {
    kph * 0.6214
}

/// Calculates mph from oss (Output Shaft Speed behind transmission)
///
/// example:
///
/// ```
/// use vehicle::formulas::mph_from_oss;
///
/// let oss = 1000.0;
/// let tire_revs_per_mile = 600.0;
/// let axle_ratio = 3.21;
/// let mph = mph_from_oss(oss, tire_revs_per_mile, axle_ratio);
/// ```

pub fn mph_from_oss(oss: f64, tire_revs_per_mile: f64, axle_ratio: f64) -> f64 {
    // oss = tire_rpm * axle_ratio
    // oss / axle_ratio = tire_rpm
    // tire_rpm / revs_per_mile * 60 = mph
    // ((oss / axle_ratio) / revs_per_mile * 60)
    oss / axle_ratio / tire_revs_per_mile * 60.0
}

/// Calculates mph from oss (Output Shaft Speed behind transmission)
///
/// example:
///
/// ```
/// use vehicle::formulas::mph_from_oss;
///
/// let mph = 100.0;
/// let tire_revs_per_mile = 600.0;
/// let axle_ratio = 3.21;
/// let oss = mph_from_oss(mph, tire_revs_per_mile, axle_ratio);
/// ```
pub fn oss_from_mph(mph: f64, tire_revs_per_mile: f64, axle_ratio: f64) -> f64 {
    // revs_per_mile * MPH = revs_per_hour
    //   632.3636 * 3 = 1897.0908 revs_per_hour
    // revs_per_hour / 60 = revs_per_minute
    //   1897.0908 / 60 = 31.61818 RPM
    // RPM x Axle Ratio = RPM
    //   31.61818 x 3.21 = 101.4943578
    ((tire_revs_per_mile * mph) / 60.0) * axle_ratio
}

/// Calculates engine RPM from transmission OSS
///
/// example:
/// ```
/// use vehicle::formulas::engine_rpm_from_oss;
///
/// let rpm = engine_rpm_from_oss(1000.0, 2.0);
/// ```
pub fn engine_rpm_from_oss(oss: f64, trans_gear_ratio: f64) -> f64 {
    oss * trans_gear_ratio
}

/// Calculates engine RPM from transmission OSS
///
/// example:
/// ```
/// use vehicle::formulas::oss_from_engine_rpm;
///
/// let oss = oss_from_engine_rpm(2000.0, 2.0);
/// ```
pub fn oss_from_engine_rpm(rpm: f64, trans_gear_ratio: f64) -> f64 {
    rpm / trans_gear_ratio
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_cm() {
        let inches = 1.0;

        let cm = to_cm(inches);

        assert!(cm == 2.54);
    }

    #[test]
    fn test_to_in() {
        let centimeters = 25.4;

        let inches = to_in(centimeters);
        assert!(inches == 10.0);
    }

    #[test]
    fn test_mph_to_kph() {
        let mph = 100.0;

        let kph = to_kph(mph);

        assert!(kph == mph * 1.609344);
    }

    #[test]
    fn test_kph_to_mph() {
        let kph = 100.0;

        let mph = to_mph(kph);

        assert!(mph == kph * 0.6214);
    }

    #[test]
    fn test_mph_from_oss() {
        let oss = 1000.0;
        let tire_revs_per_mile = 600.0;
        let axle_ratio = 3.21;

        let mph = mph_from_oss(oss, tire_revs_per_mile, axle_ratio);

        println!("{}mph", mph);
        assert!(mph == 31.15264797507788);
    }

    #[test]
    fn test_oss_from_mph() {
        let mph = 100.0;
        let tire_revs_per_mile = 600.0;
        let axle_ratio = 3.21;

        let oss = oss_from_mph(mph, tire_revs_per_mile, axle_ratio);

        println!("{}oss", oss);
        assert!(oss == 3210.0);
    }

    #[test]
    fn test_engine_rpm_from_oss() {
        let rpm = engine_rpm_from_oss(1000.0, 2.0);
        assert_eq!(rpm, 2000.0);
    }

    #[test]
    fn test_oss_from_engine_rpm() {
        let oss = oss_from_engine_rpm(2000.0, 2.0);
        assert_eq!(oss, 1000.0);
    }
}
