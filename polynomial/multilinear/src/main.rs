use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct MultilinearPolynomial {
    coefficients: Vec<f64>, // Stores all coefficients in a dense manner
    num_variables: usize,   // Number of variables in the polynomial
    max_degree: usize,      // Maximum degree per variable
}

impl MultilinearPolynomial {
    fn new(num_variables: usize, max_degree: usize) -> Self {
        let size = (max_degree + 1).pow(num_variables as u32);
        Self {
            coefficients: vec![0.0; size],
            num_variables,
            max_degree,
        }
    }
    
    fn index(&self, exponents: &[u32]) -> usize {
        exponents.iter().enumerate().fold(0, |acc, (i, &exp)| acc * (self.max_degree + 1) + exp as usize)
    }
    
    fn add_term(&mut self, exponents: Vec<u32>, coefficient: f64) {
        if exponents.len() == self.num_variables {
            let idx = self.index(&exponents);
            self.coefficients[idx] = coefficient;
        }
    }
    
    fn evaluate(&self, variables: &[f64]) -> f64 {
        let mut result = 0.0;
        for (idx, &coefficient) in self.coefficients.iter().enumerate() {
            if coefficient != 0.0 {
                let mut term_value = coefficient;
                let mut index = idx;
                for var in (0..self.num_variables).rev() {
                    let exp = index % (self.max_degree + 1);
                    term_value *= variables[var].powi(exp as i32);
                    index /= self.max_degree + 1;
                }
                result += term_value;
            }
        }
        result
    }
    
    fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for i in 0..self.coefficients.len() {
            result.coefficients[i] += other.coefficients[i];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_evaluate() {
        let mut poly = MultilinearPolynomial::new(2, 2);
        poly.add_term(vec![1, 0], 2.0);
        poly.add_term(vec![0, 1], 3.0);
        assert_eq!(poly.evaluate(&[2.0, 3.0]), 2.0 * 2.0 + 3.0 * 3.0);
    }
    
    #[test]
    fn test_addition() {
        let mut poly1 = MultilinearPolynomial::new(2, 2);
        poly1.add_term(vec![1, 0], 2.0);
        let mut poly2 = MultilinearPolynomial::new(2, 2);
        poly2.add_term(vec![1, 0], 3.0);
        let result = poly1.add(&poly2);
        let mut expected = MultilinearPolynomial::new(2, 2);
        expected.add_term(vec![1, 0], 5.0);
        assert_eq!(result, expected);
    }
}

fn main() {
    let mut poly1 = MultilinearPolynomial::new(3, 2);
    poly1.add_term(vec![1, 0, 2], 3.0);
    poly1.add_term(vec![0, 2, 1], 2.0);
    
    let mut poly2 = MultilinearPolynomial::new(3, 2);
    poly2.add_term(vec![1, 0, 2], 1.5);
    poly2.add_term(vec![0, 1, 1], 4.0);
    
    let poly_sum = poly1.add(&poly2);
    println!("Polynomial sum: {:?}", poly_sum);
    
    let result = poly1.evaluate(&[1.0, 2.0, 3.0]);
    println!("Evaluation result: {}", result);
}
