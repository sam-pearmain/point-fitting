use crate::utils::error::FittingError;

pub enum PolyInterpolationMethod {
    Lagrange,
    Newton,
}

pub struct Polynomial {
    degree: u16,
    coefficients: Vec<f64>,
}

pub fn interpolate(
    x: Vec<f64>,
    y: Vec<f64>,
    method: PolyInterpolationMethod,
) -> Result<Polynomial, FittingError> {

}

pub fn lagrange_interpolation() {
    // todo
}

pub fn newton_interpolation() {
    // todo
}