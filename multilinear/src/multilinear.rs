// use std::collections::HashMap;
// use ark_ff::PrimeField;
// use ark_ff::BigInteger;

// #[derive(Debug, Clone, PartialEq)]
// pub struct MultilinearEvalForm<F: PrimeField> {
//     pub evaluated_values: Vec<F>,
// }

// impl<F: PrimeField> MultilinearEvalForm<F> {
//     pub fn new(evaluated_values: &Vec<F>) -> Self {
//         assert!(
//             evaluated_values.len().is_power_of_two(),
//             "Number of evaluated values must be a power of 2"
//         );
//         Self {
//             evaluated_values: evaluated_values.to_vec(),
//         }
//     }

//     pub fn partial_evaluate(&self, eval_var: usize, value: F) -> Vec<F> {
//         let poly_size = self.evaluated_values.len();
//         assert!(
//             poly_size.is_power_of_two(),
//             "Polynomial length must be a power of 2"
//         );

//         let expected_poly_size = poly_size / 2;
//         let mut result_poly = vec![F::zero(); expected_poly_size];

//         for i in 0..expected_poly_size {
//             let first = self.evaluated_values[i * 2];
//             let second = self.evaluated_values[i * 2 + 1];
//             result_poly[i] = first + value * (second - first);
//         }

//         assert_eq!(
//             result_poly.len(),
//             expected_poly_size,
//             "Result polynomial size mismatch"
//         );

//         result_poly
//     }

//     pub fn evaluate(&self, values: &Vec<F>) -> F {
//         assert_eq!(
//             values.len() as u32,
//             self.number_of_variables(),
//             "Number of values must match number of variables"
//         );

//         let mut x_poly = self.evaluated_values.clone();
//         for &value in values.iter() {
//             x_poly = MultilinearEvalForm { evaluated_values: x_poly }
//                 .partial_evaluate(0, value);
//         }

//         x_poly[0]
//     }

//     pub fn convert_to_bytes(&self) -> Vec<u8> {
//         let mut bytes = Vec::new();
//         for value in &self.evaluated_values {
//             bytes.extend(value.into_bigint().to_bytes_be());
//         }
//         bytes
//     }

//     pub fn number_of_variables(&self) -> u32 {
//         self.evaluated_values.len().ilog2()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ark_bn254::Fr;

//     #[test]
//     fn test_partial_evaluate() {
//         let polynomial = vec![Fr::from(0), Fr::from(2), Fr::from(4), Fr::from(6)];
//         let mel = MultilinearEvalForm::new(&polynomial);

//         let result = mel.partial_evaluate(0, Fr::from(1));
//         assert_eq!(result.len(), 2);
//         assert_eq!(result[0], Fr::from(2)); // (0 + 1 * (2 - 0)) = 2
//         assert_eq!(result[1], Fr::from(6)); // (4 + 1 * (6 - 4)) = 6
//     }

//     #[test]
//     fn test_evaluate() {
//         let polynomial = vec![Fr::from(0), Fr::from(0), Fr::from(5), Fr::from(7)];
//         let mel = MultilinearEvalForm::new(&polynomial);

//         let eval_point = vec![Fr::from(1), Fr::from(1)];
//         let result = mel.evaluate(&eval_point);
//         assert_eq!(result, Fr::from(7));
//     }

//     #[test]
//     #[should_panic(expected = "Number of values must match number of variables")]
//     fn test_evaluate_wrong_number_of_variables() {
//         let polynomial = vec![Fr::from(0), Fr::from(0), Fr::from(5), Fr::from(7)];
//         let mel = MultilinearEvalForm::new(&polynomial);
//         let eval_point = vec![Fr::from(1)];
//         mel.evaluate(&eval_point);
//     }

//     #[test]
//     #[should_panic(expected = "Number of evaluated values must be a power of 2")]
//     fn test_invalid_polynomial_size() {
//         let polynomial = vec![Fr::from(0), Fr::from(0), Fr::from(5)];
//         MultilinearEvalForm::new(&polynomial);
//     }
// }
use ark_ff::PrimeField;
use ark_ff::BigInteger;

#[derive(Debug, Clone, PartialEq)]
pub struct MultilinearEvalForm<F: PrimeField> {
    pub evaluated_values: Vec<F>,
}

impl<F: PrimeField> MultilinearEvalForm<F> {
    pub fn new(evaluated_values: &Vec<F>) -> Self {
        assert!(
            evaluated_values.len().is_power_of_two(),
            "Number of evaluated values must be a power of 2"
        );
        Self {
            evaluated_values: evaluated_values.to_vec(),
        }
    }

    pub fn partial_evaluate(&self, eval_var: usize, value: F) -> Vec<F> {
        let poly_size = self.evaluated_values.len();
        assert!(
            poly_size.is_power_of_two(),
            "Polynomial length must be a power of 2"
        );
        
        let num_vars = self.number_of_variables() as usize;
        assert!(
            eval_var < num_vars,
            "Evaluation variable index out of bounds"
        );

        // For sumcheck, we evaluate from most significant variable (leftmost) to least significant
        let actual_var = num_vars - 1 - eval_var;
        let expected_poly_size = poly_size / 2;
        let mut result_poly = vec![F::zero(); expected_poly_size];

        for i in 0..expected_poly_size {
            // Calculate indices based on the variable we're evaluating
            let low_idx = i * 2;
            let high_idx = low_idx + 1;
            
            let low = self.evaluated_values[low_idx];
            let high = self.evaluated_values[high_idx];
            result_poly[i] = low + (high - low) * value;
        }

        result_poly
    }

    pub fn evaluate(&self, values: &Vec<F>) -> F {
        assert_eq!(
            values.len() as u32,
            self.number_of_variables(),
            "Number of values must match number of variables"
        );

        let mut current_poly = self.clone();
        
        for (i, &value) in values.iter().enumerate() {
            current_poly = MultilinearEvalForm::new(&current_poly.partial_evaluate(i, value));
        }

        current_poly.evaluated_values[0]
    }

    pub fn convert_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.evaluated_values.len() * 32);
        for value in &self.evaluated_values {
            bytes.extend(value.into_bigint().to_bytes_be());
        }
        bytes
    }

    pub fn number_of_variables(&self) -> u32 {
        self.evaluated_values.len().ilog2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

    #[test]
    fn test_partial_evaluate_order() {
        // Test polynomial representing f(x,y) = x + 2y
        // Values at (x,y): f(0,0)=0, f(0,1)=2, f(1,0)=1, f(1,1)=3
        let polynomial = vec![
            Fr::from(0), // (0,0)
            Fr::from(2), // (0,1)
            Fr::from(1), // (1,0)
            Fr::from(3), // (1,1)
        ];
        let mel = MultilinearEvalForm::new(&polynomial);

        // Evaluate x=1 first (first round in sumcheck)
        let after_x = mel.partial_evaluate(0, Fr::from(1));
        assert_eq!(after_x.len(), 2);
        // Should get f(1,0)=1 and f(1,1)=3
        assert_eq!(after_x[0], Fr::from(1));
        assert_eq!(after_x[1], Fr::from(3));

        // Then evaluate y=1
        let mel_after_x = MultilinearEvalForm::new(&after_x);
        let final_result = mel_after_x.partial_evaluate(1, Fr::from(1));
        assert_eq!(final_result[0], Fr::from(3)); // f(1,1) = 3
    }

    #[test]
    fn test_evaluate() {
        let polynomial = vec![Fr::from(0), Fr::from(2), Fr::from(1), Fr::from(3)];
        let mel = MultilinearEvalForm::new(&polynomial);

        let eval_point = vec![Fr::from(1), Fr::from(1)];
        let result = mel.evaluate(&eval_point);
        assert_eq!(result, Fr::from(3));
    }

    #[test]
    #[should_panic(expected = "Number of values must match number of variables")]
    fn test_evaluate_wrong_number_of_variables() {
        let polynomial = vec![Fr::from(0), Fr::from(2), Fr::from(1), Fr::from(3)];
        let mel = MultilinearEvalForm::new(&polynomial);
        let eval_point = vec![Fr::from(1)];
        mel.evaluate(&eval_point);
    }

    #[test]
    #[should_panic(expected = "Evaluation variable index out of bounds")]
    fn test_invalid_eval_var() {
        let polynomial = vec![Fr::from(0), Fr::from(2), Fr::from(1), Fr::from(3)];
        let mel = MultilinearEvalForm::new(&polynomial);
        mel.partial_evaluate(2, Fr::from(1));
    }
}