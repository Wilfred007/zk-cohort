use crate::gates::{Gate, Op};
use ark_ff::PrimeField;


#[derive(Debug)]


struct Layer {
    gates: Vec<Gate>,
}

impl Layer {
    fn new(gates: Vec<Gate>) -> Self {
        Layer {gates}
    }


    fn evaluate<F: PrimeField>(&self, inputs: Vec<F>) -> Vec<F> {
        let mut output = vec![F::from(0); self.gates.len()];
        for gate in &self.gates {
            gate.evaluate(&inputs, &mut output);
        }
        output
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn layer_creation() {
        let gate1 = Gate::new(0,1,2, Op::Add);
        let gate2 = Gate::new(1,2,3, Op::Mul);
        let layer_one = Layer::new(vec![gate1, gate2]);
        assert_eq!(layer_one.gates.len(), 2);

        println!("Layer one: {:?}", layer_one);
        assert_eq!(layer_one.gates.len(), 2);
    }

    // #[test]

}