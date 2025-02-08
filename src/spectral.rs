use crate::matrix::{self, Matrix};
use crate::graph::{self, AdjMatrix};

// Check if the graph represented by this AdjMatrix is integral
pub fn is_integral(adjm: AdjMatrix) -> Result<bool, String> {
    let matr: Matrix = adjm.try_into()?;
    let poly = characteristic_polynomial(&matr)?;
    Ok(is_factorable(&poly))
}

/// Compute the characteristic polynomial of a square matrix
/// using the Faddeev-LeVerrier algorithm
pub fn characteristic_polynomial(matrix_a: &Matrix) -> Result<Vec<i64>, String> {
    let n = matrix_a.get_n();
    let mut coeffs = vec![0; (n as usize) + 1];
    coeffs[n as usize] = 1;
    let mut matrix_b = Matrix::identity(n)?;
    
    for i in 1..=n {
        let multiplied = matrix::multiply(&matrix_a, &matrix_b)?;
        let trace = multiplied.trace()?;

        if trace % (i as i64) != 0 {
            return Err("something went really wrong :(".to_string());
        }

        let coeff = -trace / (i as i64);
        coeffs[(n as usize) - (i as usize)] = coeff;

        let matrix_c = Matrix::zeroed(n)?;
        for i in 0..n {
            matrix_c.set(i, i, coeff)?;
        }

        matrix_b = matrix::add(&multiplied, &matrix_c)?;
    }

    Ok(coeffs)
}

pub fn divisors(number: i64) -> Vec<i64> {
    if number == 0 {
        return vec![0];
    }
    let mut positive_divs = Vec::new();
    let mut negative_divs = Vec::new();
    let number = if number > 0 { number } else { -number };
    for div in 1..=number {
        if number % div == 0 {
            negative_divs.push(-div);
            positive_divs.push(div);
        }
    }
    negative_divs.reverse();
    negative_divs.extend(positive_divs);
    negative_divs
}

/// Divides polynomial by (x - r)
/// poly[i] represents the coefficient corresponding to x^i
pub fn synthetic_division(poly: &[i64], r: i64) -> Option<Vec<i64>> {
    let n = poly.len();
    if n == 0 {
        return None;
    }
    let mut result = vec![0; n - 1];

    result[n - 2] = poly[n - 1];
    for i in 1..n - 1 {
        result[n - 2 - i] = r * result[n - 1 - i] + poly[n - 1 - i];
    }

    let remainder = r * result[0] + poly[0];
    if remainder != 0 {
        return None;
    }

    Some(result.to_vec())
}

/// Returns true if the polynomial can be factored into
/// expressions (x - n), where n is an integer
pub fn is_factorable(poly: &[i64]) -> bool {
    if poly.len() == 0 {
        return false;
    }

    if poly.len() == 1 {
        return true;
    }

    if poly[0] == 0 {
        if let Some(quotient) = synthetic_division(poly, 0) {
            return is_factorable(&quotient);
        } else {
            return false;
        }
    }

    // Any integral root must be a divisor of the constant term
    for potential_root in divisors(poly[0]) {
        if let Some(quotient) = synthetic_division(poly, potential_root) {
            if is_factorable(&quotient) {
                return true;
            }
        }
    }
    false
}
