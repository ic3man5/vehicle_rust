//! Trigonometry Functions with Transformations
// https://www.integral-domain.org/lwilliams/Applets/precalculus/trigTransformations.php

//use std::ops::{Add, Div, Mul, Sub, AddAssign, MulAssign, DivAssign, SubAssign};

/// Transformations for Trigonometry Functions
#[derive(Debug)]
pub struct Function {
    pub x: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl Function {
    /// Creates a new function with amplitute, period, phase shift, and vertical shift parameters
    /// 
    /// example:
    /// 
    /// ```
    /// use vehicle::trig::Function;
    /// 
    /// let mut function = Function::new(2.0, Some(1.0), Some(1.0), Some(0.0), Some(0.0));
    /// assert_eq!(function.sin(), 2.0_f64.sin());
    /// println!("sin: {}", function.sin());
    /// ```
    pub fn new(x: f64, amplitude: Option<f64>, period: Option<f64>, phase_shift: Option<f64>, vertical_shift: Option<f64>) -> Self {
        let a: f64 = match amplitude {
            Some(value) => value,
            None => 1.0,
        };
        let b: f64 = match period {
            Some(value) => value,
            None => 1.0,
        };
        let c: f64 = match phase_shift {
            Some(value) => value,
            None => 0.0,
        };
        let d: f64 = match vertical_shift {
            Some(value) => value,
            None => 0.0,
        };

        Self {
            x,
            a,
            b,
            c,
            d,
        }
    }

    pub fn sin(&self) -> f64 {
        // y = A sin(Bx - C) + D
        self.a * (self.b*self.x-self.c).sin() + self.d
    }

    pub fn cos(&self) -> f64 {
        self.a * (self.b*self.x-self.c).cos() + self.d
    }

    pub fn tan(&self) -> f64 {
        self.a * (self.b*self.x-self.c).tan() + self.d
    }

    pub fn asin(&self) -> f64 {
        self.a * (self.b*self.x-self.c).asin() + self.d
    }

    pub fn acos(&self) -> f64 {
        self.a * (self.b*self.x-self.c).acos() + self.d
    }

    pub fn atan(&self) -> f64 {
        self.a * (self.b*self.x-self.c).atan() + self.d
    }

    /// Sets the Amplitude (a) of the function. (y = A sin(Bx - C) + D)
    /// 
    /// 
    /// The amplitude of a sinusoidal trig function (sine or cosine) is it's 'height,' 
    /// the distance from the average value of the curve to its maximum (or minimum) value.
    /// The other trig functions (tangent, cotangent, secant, and cosecant) do not have an amplitude, 
    /// but multiplication by A will affect their steepness. Note that a negative value will 
    /// flip the graph of any function across the x-axis.
    pub fn set_amplitude(&mut self, a: f64) {
        self.a = a;
    } 

    /// Sets the Period (b) of the function. (y = A sin(Bx - C) + D)
    /// 
    /// 
    /// The period of any trig function is the length of one cycle. sin(x), cos(x), sec(x), and csc(x) 
    /// all have a period of 2π, while tan(x) and cot(x) have a period of π.
    pub fn set_period(&mut self, p: f64) {
        self.b = p;
    } 

    /// Sets the Phase Shift (c) of the function. (y = A sin(Bx - C) + D)
    /// 
    /// 
    /// The phase shift of a trigonometric function refers to its horizontal shift to the right. 
    /// A phase shift results from adding a value to the variable before the evaluating the function.
    /// When positive, the graph will appear to shift to the right. When negative, the graph will shift to the left.
    pub fn set_phase_shift(&mut self, ps: f64) {
        self.c = ps;
    }

    /// Sets the Vertical Shift (d) of the function. (y = A sin(Bx - C) + D)
    /// 
    /// 
    /// The Vertical Shift will translate its graph vertically. If positive, the graph will shift up by a factor 
    /// specified; if negative, the graph will shift down.
    pub fn set_vertical_shift(&mut self, vs: f64) {
        self.d = vs;
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal() {
        let function = Function::new(2.0, Some(1.0), Some(1.0), Some(0.0), Some(0.0));
        assert_eq!(function.sin(), 2.0_f64.sin());
        assert_eq!(function.cos(), 2.0_f64.cos());
        assert_eq!(function.tan(), 2.0_f64.tan());
        //assert_eq!(function.asin(), 2.0_f64.asin());
        //assert_eq!(function.acos(), 2.0_f64.acos());
        //assert_eq!(function.atan(), 2.0_f64.atan());
    }
}