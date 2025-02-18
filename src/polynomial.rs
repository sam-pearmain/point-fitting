#![allow(dead_code)]

use crate::utils::error::InterpolationError;

pub enum PolyInterpolationMethod {
    Lagrange,
    Newton,
}

pub struct Polynomial {
    degree: usize,
    coefficients: Vec<f64>,
}

pub fn interpolate(
    x: Vec<f64>,
    y: Vec<f64>,
    method: PolyInterpolationMethod,
) -> Result<Polynomial, InterpolationError> {
    if x.len() != y.len() {
        return Err(InterpolationError::UnequalArrayLengths);
    }

    match method {
        PolyInterpolationMethod::Lagrange => lagrange_interpolation(&x, &y),
        PolyInterpolationMethod::Newton => newton_interpolation(&x, &y),
    }
}

fn lagrange_interpolation(x: &[f64], y: &[f64]) -> Result<Polynomial, InterpolationError> {
    let n: usize = x.len();
    let mut coefficients: Vec<f64> = vec![0.0; n];

    for i in 0..n {
        let mut term_coefficients: Vec<f64> = vec![1.0];

        for j in 0..n {
            if i != j {
                let denominator = x[i] - x[j];
                if denominator.abs() < f64::EPSILON {
                    return Err(InterpolationError::DuplicateValues);
                }

                term_coefficients = term_coefficients
                    .iter()
                    .chain(std::iter::once(&0.0))
                    .zip(term_coefficients.iter().skip(1).chain(std::iter::once(&0.0)))
                    .map(|(a, b)| a - b * x[j] / denominator)
                    .collect();
            }
        }

        for coefficient in &mut term_coefficients {
            *coefficient *= y[i];
        }

        coefficients = coefficients
            .iter()
            .zip(term_coefficients.iter().chain(std::iter::repeat(&0.0)))
            .map(|(a, b)| a + b)
            .collect();
    }

    Ok(Polynomial{
        degree: n - 1,
        coefficients
    })
}

fn newton_interpolation(x: &[f64], y: &[f64]) -> Result<Polynomial, InterpolationError> {
    let n = x.len();
    let mut divided_differences = y.to_vec();

    // compute divided differences
    for j in 0..n {
        for i in (j..n).rev() {
            let denominator = x[i] - x[i - j];
            if denominator.abs() < f64::EPSILON {
                return Err(InterpolationError::DuplicateValues);
            }

            divided_differences[i] = (divided_differences[i] - divided_differences[i - 1]) / denominator;
        }
    }

    //construct the polynomial
    let mut coefficients = vec![0.0, n as f64];
    coefficients[0] = divided_differences[0];

    for i in 1..n {
        let mut term_coefficients = vec![1.0];

        for j in 0..i {
            // multiply term by (x - x[j])
            term_coefficients = term_coefficients
                .iter()
                .chain(std::iter::once(&0.0))
                .zip(term_coefficients.iter().skip(1).chain(std::iter::once(&0.0)))
                .map(|(a, b)| a - b * x[j])
                .collect(); 

            // scale the terms by the divided difference
            for coefficient in &mut term_coefficients {
                *coefficient *= divided_differences[i];
            }

            // add the term coefficients to the polynomial
            coefficients = coefficients
            .iter()
            .zip(term_coefficients.iter().chain(std::iter::repeat(&0.0)))
            .map(|(a, b)| a + b)
            .collect();
        }
    }

    Ok(Polynomial{
        degree: n - 1,
        coefficients,
    })
}

pub fn polynomial_regression(x: &[f64], y: &[f64], degree: usize) -> Result<Polynomial, InterpolationError> {
    if x.len() != y.len() {
        return Err(InterpolationError::UnequalArrayLengths);
    }

    Ok(Polynomial{
        degree: degree,
        coefficients: coefficients
    })
}