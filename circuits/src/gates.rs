use ark_ff::PrimeField;



#[derive(Debug)]
pub struct Gate {
    input1: u32,
    input2: u32,
    output: u32,
    op: Op,
}

#[derive(Debug)]
pub enum Op {
    Add,
    Mul,
}

impl Gate {
    pub fn new(input1: u32, input2: u32, output:u32, op: Op) -> Self {
        Gate {
            input1,
            input2,
            output,
            op,
        }
    }

    pub fn evaluate<F: PrimeField>(&self, inputs: &Vec<F>, output: &mut Vec<F>) {
        let input1 = inputs[self.input1 as usize]; // Ensure we cast input1 to usize for indexing
        let input2 = inputs[self.input2 as usize]; // Same for input2

        let result = match self.op {
            Op::Add => input1 + input2,
            Op::Mul => input1 * input2,
        };

        output[self.output as usize] += result;
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ark_bn254::Fq;

//     #[test]
//     fn test_gate_creation() {
//         let gate = Gate::new(1, 2, 3, Op::Add); // Creating a sample gate
//         assert_eq!(gate.input1, 1);
//         assert_eq!(gate.input2, 2);
//         assert_eq!(gate.output, 3);
//         assert_eq!(matches!(gate.op, Op::Add), true);
//     }

//     #[test]
//     fn test_gate_evaluation() {
//         // Creating a sample gate and testing the evaluation
//         let gate = Gate::new(0, 1, 2, Op::Add);
//         let inputs: Vec<Fq> = vec![Fq::from(3), Fq::from(4)]; // Inputs as Fq
//         let mut output: Vec<Fq> = vec![Fq::from(0), Fq::from(0), Fq::from(0)];

//         gate.evaluate(&inputs, &mut output);

//         assert_eq!(output[2], Fq::from(7)); // 3 + 4 = 7, so output[2] should be 7
//     }
// }
