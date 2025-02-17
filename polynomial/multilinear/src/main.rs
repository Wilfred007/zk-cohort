use std::collections::HashMap;
use ark_ff::PrimeField;
use ark_ff::BigInteger;

#[derive(Debug, Clone, PartialEq)]
struct MultilinearEvalForm<F: PrimeField> {
    evaluated_values: Vec<F> // Values at hypercube corners
}

impl<F: PrimeField> MultilinearEvalForm<F> {
    fn new(evaluated_values: &Vec<F>) -> Self {
        assert!(evaluated_values.len().is_power_of_two(), 
            "Number of evaluated values must be a power of 2");
        Self {
            evaluated_values: evaluated_values.to_vec()
        }
    }

    fn evaluate(&self, values: &Vec<F>) -> F {
        assert_eq!(
            values.len() as u32,
            self.number_of_variables(),
            "Number of values must match number of variables"
        );
        
        let mut x_poly = self.evaluated_values.clone();
        for (i, &value) in values.iter().enumerate() {
            x_poly = partial_evaluate(&x_poly, i, value);
        }
        x_poly[0]
    }

    fn convert_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for value in &self.evaluated_values {
            bytes.extend(value.into_bigint().to_bytes_be());
        }
        bytes
    }

    fn number_of_variables(&self) -> u32 {
        self.evaluated_values.len().ilog2()
    }
}

fn partial_evaluate<F: PrimeField>(polynomial: &Vec<F>, eval_var: usize, value: F) -> Vec<F> {
    assert!(polynomial.len().is_power_of_two(),
        "Polynomial length must be a power of 2");
    
    let poly_size = polynomial.len();
    let expected_poly_size = poly_size / 2;
    let number_of_variables = poly_size.ilog2() as usize;
    let power = number_of_variables - 1 - eval_var;
    let step = 1 << power;

    let mut result_poly: Vec<F> = Vec::with_capacity(expected_poly_size);
    
    let mut i = 0;
    while i < poly_size {
        let first_pair = polynomial[i];
        let second_pair = polynomial[i | step];
        result_poly.push(first_pair + (value * (second_pair - first_pair)));
        
        i = if (i + 1) % step == 0 {
            i + 1 + step
        } else {
            i + 1
        };
    }

    assert_eq!(result_poly.len(), expected_poly_size, 
        "Result polynomial size mismatch");
    result_poly
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;  // Using Fr instead of Fq for consistency

    #[test]
    fn test_partial_evaluate() {
        let polynomial = vec![Fr::from(0), Fr::from(0), Fr::from(5), Fr::from(7)];
        let mel = MultilinearEvalForm::new(&polynomial);

        // Test partial evaluation for first variable
        let result = partial_evaluate(&polynomial, 0, Fr::from(1));
        assert_eq!(result.len(), 2);

        // Test partial evaluation for second variable
        let result = partial_evaluate(&polynomial, 1, Fr::from(1));
        assert_eq!(result.len(), 2);

        // Test full evaluation
        let eval_point = vec![Fr::from(1), Fr::from(1)];
        let result = mel.evaluate(&eval_point);
        assert_eq!(result, Fr::from(7));
    }

    #[test]
    #[should_panic(expected = "Number of values must match number of variables")]
    fn test_evaluate_wrong_number_of_variables() {
        let polynomial = vec![Fr::from(0), Fr::from(0), Fr::from(5), Fr::from(7)];
        let mel = MultilinearEvalForm::new(&polynomial);
        let eval_point = vec![Fr::from(1)];  // Only one value when we need two
        mel.evaluate(&eval_point);
    }

    #[test]
    #[should_panic(expected = "Number of evaluated values must be a power of 2")]
    fn test_invalid_polynomial_size() {
        let polynomial = vec![Fr::from(0), Fr::from(0), Fr::from(5)];  // Size 3 is not a power of 2
        MultilinearEvalForm::new(&polynomial);
    }
}