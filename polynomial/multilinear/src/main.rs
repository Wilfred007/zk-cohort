use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
struct MultilinearPolynomial {
    terms: HashMap<Vec<u32>, f64>, // Maps exponent vectors to coefficients
}

impl MultilinearPolynomial {
    fn new() -> Self {
        Self {
            terms: HashMap::new(),
        }
    }
    
    fn add_term(&mut self, exponents: Vec<u32>, coefficient: f64) {
        if coefficient != 0.0 {
            self.terms.insert(exponents, coefficient);
        }
    }
    
    fn evaluate(&self, variables: &[f64]) -> f64 {
        let mut result = 0.0;
        for (exponents, &coefficient) in &self.terms {
            let term_value = exponents.iter().enumerate()
                .map(|(i, &exp)| variables.get(i).unwrap_or(&1.0).powi(exp as i32))
                .product::<f64>();
            result += coefficient * term_value;
        }
        result
    }
    
    fn partial_evaluate(&self, variables: &[Option<f64>]) -> Self {
        let mut new_poly = MultilinearPolynomial::new();
        for (exponents, &coefficient) in &self.terms {
            let mut new_exponents = vec![0; exponents.len()];
            let mut new_coefficient = coefficient;
            for (i, &exp) in exponents.iter().enumerate() {
                if let Some(value) = variables.get(i).and_then(|&x| x) {
                    new_coefficient *= value.powi(exp as i32);
                } else {
                    new_exponents[i] = exp;
                }
            }
            new_poly.add_term(new_exponents, new_coefficient);
        }
        new_poly
    }
    
    fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (exponents, &coefficient) in &other.terms {
            result.terms.entry(exponents.clone()).or_insert(0.0);
            *result.terms.get_mut(exponents).unwrap() += coefficient;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_evaluate() {
        let mut poly = MultilinearPolynomial::new();
        poly.add_term(vec![1, 0], 2.0); // 2 * x_1
        poly.add_term(vec![0, 1], 3.0); // 3 * x_2
        assert_eq!(poly.evaluate(&[2.0, 3.0]), 2.0 * 2.0 + 3.0 * 3.0);
    }
    
    #[test]
    fn test_partial_evaluate() {
        let mut poly = MultilinearPolynomial::new();
        poly.add_term(vec![1, 1], 4.0); // 4 * x_1 * x_2
        let partial_poly = poly.partial_evaluate(&[Some(2.0), None]);
        let expected = {
            let mut p = MultilinearPolynomial::new();
            p.add_term(vec![0, 1], 8.0); // 8 * x_2
            p
        };
        assert_eq!(partial_poly, expected);
    }
    
    #[test]
    fn test_addition() {
        let mut poly1 = MultilinearPolynomial::new();
        poly1.add_term(vec![1, 0], 2.0);
        let mut poly2 = MultilinearPolynomial::new();
        poly2.add_term(vec![1, 0], 3.0);
        let result = poly1.add(&poly2);
        let mut expected = MultilinearPolynomial::new();
        expected.add_term(vec![1, 0], 5.0);
        assert_eq!(result, expected);
    }
}

fn main() {
    let mut poly1 = MultilinearPolynomial::new();
    poly1.add_term(vec![1, 0, 2], 3.0); // 3 * x_1 * x_3^2
    poly1.add_term(vec![0, 2, 1], 2.0); // 2 * x_2^2 * x_3
    
    let mut poly2 = MultilinearPolynomial::new();
    poly2.add_term(vec![1, 0, 2], 1.5); // 1.5 * x_1 * x_3^2
    poly2.add_term(vec![0, 1, 1], 4.0); // 4 * x_2 * x_3
    
    let poly_sum = poly1.add(&poly2);
    println!("Polynomial sum: {:?}", poly_sum);
    
    let result = poly1.evaluate(&[1.0, 2.0, 3.0]); // Evaluates at x_1=1, x_2=2, x_3=3
    println!("Evaluation result: {}", result);
    
    let partial_poly = poly1.partial_evaluate(&[Some(1.0), None, Some(3.0)]);
    println!("Partial evaluation: {:?}", partial_poly);
}
