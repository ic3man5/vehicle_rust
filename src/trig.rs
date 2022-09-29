//! Trigonometry Functions with Transformations
// https://www.integral-domain.org/lwilliams/Applets/precalculus/Transformation.php

use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
/// Trigonometry Functions Types
pub enum FunctionType {
    Sine,
    Cosine,
    Tangent,
    Cotangent,
    Secant,
    Cosecant,
}

#[derive(Debug)]
/// Transformations for Trigonometry Functions
pub enum Transformation<T: Copy> {
    /// The amplitude of a sinusoidal trig function (sine or cosine) is it's 'height,' 
    /// the distance from the average value of the curve to its maximum (or minimum) value.
    /// The other trig functions (tangent, cotangent, secant, and cosecant) do not have an amplitude, 
    /// but multiplication by A will affect their steepness. Note that a negative value will 
    /// flip the graph of any function across the x-axis.
    Amplify(T),
    /// The period of any trig function is the length of one cycle. sin(x), cos(x), sec(x), and csc(x) 
    /// all have a period of 2π, while tan(x) and cot(x) have a period of π.
    Period(T),
    /// The phase shift of a trigonometric function refers to its horizontal shift to the right. 
    /// A phase shift results from adding a value to the variable before the evaluating the function.
    /// When positive, the graph will appear to shift to the right. When negative, the graph will shift to the left.
    PhaseShift(T),
    /// The Vertical Shift will translate its graph vertically. If positive, the graph will shift up by a factor 
    /// specified; if negative, the graph will shift down.
    VerticalShift(T),
}

#[derive(Debug)]
/// Represents a Trigonometry Function that allows easy transformations
pub struct Function<T: Copy> {
    pub func_type: FunctionType,
    mods: Vec<Transformation<T>>,
}

impl<T: Copy> Function<T>
where
    T: Mul<T> + Mul<Output = T>,
    T: Div<T> + Div<Output = T>,
    T: Add<T> + Add<Output = T>,
    T: Sub<T> + Sub<Output = T>,
{
    pub fn new(func_type: FunctionType) -> Self {
        let mods = Vec::new();
        Self {
            func_type,
            mods,
        }
    }

    /// Add a Transformation to the function
    /// 
    /// example:
    /// ```
    /// use vehicle::trig::{Function, FunctionType, Transformation};
    /// 
    /// let mut sin = Function::new(FunctionType::Sine);
    /// sin.add(Transformation::Amplify(2.0))
    ///     .add(Transformation::Period(3.0))
    ///     .add(Transformation::PhaseShift(1.2));
    /// println!("{}", sin.calc_y(0.0));
    /// ```
    pub fn add(&mut self, transformation: Transformation<T>) -> &mut Self {
        self.mods.push(transformation);
        self
    }

    pub fn amplify(&mut self, a: T) -> &mut Self {
        self.mods.push(Transformation::Amplify(a));
        self
    }

    pub fn period(&mut self, p: T) -> &mut Self {
        self.mods.push(Transformation::Period(p));
        self
    }

    pub fn phase_shift(&mut self, ps: T) -> &mut Self {
        self.mods.push(Transformation::PhaseShift(ps));
        self
    }

    pub fn vertical_shift(&mut self, vs: T) -> &mut Self {
        self.mods.push(Transformation::VerticalShift(vs));
        self
    }

    pub fn calc_x(&self, y: T) -> T {
        y
    }
    
    pub fn calc_y(&self, x: T) -> T {
        x
    }

}