/// # Degree
/// A geometrical degree
/// ## Fields
/// - `degree`
/// - `minute`
/// - `second`
pub struct Degree {
    pub degree: i16,
    pub minute: i16,
    pub second: i16
}

impl Degree {
    pub const fn new(degree: i16, minute: i16, second: i16) -> Self {
        Self { degree, minute, second }
    }

    /// Convert a degree to a floating point number.
    /// For example:
    /// `180° 30' 0"` -> `180.5°`
    pub fn to_float(&self) -> f64 {
        self.degree as f64 + self.minute as f64 / 60. + self.second as f64 / 3600.
    }

    /// Create a degree from a floating point number.
    /// For example:
    /// `180.5°` -> `180° 30' 0"`
    pub fn from_float(float: f64) -> Self {
        // Convert the float to seconds
        let seconds: i128 = (float * 3600.).round() as i128;

        // Convert the number of seconds to degrees and minutes
        let degree: i16 = (seconds / 3600) as i16;
        let minute: i16 = ((seconds - degree as i128 * 3600) / 60) as i16;

        // Subtract the degrees and minutes from the seconds
        let second: i16 = (seconds - degree as i128 * 3600 - minute as i128 * 60) as i16;

        Self { degree, minute, second }
    }

    /// Convert the degrees to radians
    pub fn to_radian(&self) -> f64 {
        self.to_float().to_radians()
    }

    /// Convert radians to degrees
    pub fn from_radian(radian: f64) -> Self {
        Self::from_float(radian.to_degrees())
    }

    pub fn sin(&self) -> f64 { self.to_radian().sin() }
    pub fn cos(&self) -> f64 { self.to_radian().cos() }
    pub fn tan(&self) -> f64 { self.to_radian().tan() }
    pub fn asin(ratio: f64) -> Self { Self::from_radian(ratio.asin()) }
    pub fn acos(ratio: f64) -> Self { Self::from_radian(ratio.acos()) }
    pub fn atan(ratio: f64) -> Self { Self::from_radian(ratio.atan()) }
}

impl std::fmt::Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}° {}' {}\"", self.degree, self.minute, self.second)
    }
}

impl std::fmt::Debug for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_radian())
    }
}

impl std::fmt::Binary for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:b} {:b} {:b}", self.degree, self.minute, self.second)
    }
}

impl Clone for Degree {
    fn clone(&self) -> Self {
        Self {
            degree: self.degree,
            minute: self.minute,
            second: self.second
        }
    }
}

impl Copy for Degree { }