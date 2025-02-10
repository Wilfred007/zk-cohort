use ark_ff::PrimeField;
use rand;
use polynomial::univariate::DensePolynomial;

fn split_secret(secret: F, n:u32, quorum: u32) -> Vec(<f,f>){
    let mut yValues: Vec<F> = vec![secret];


    let mut rng = rand::thread_rng();


    for _i in 0..quorum - 1 {
        let mut y: F;

        if y != secret && !yValues.contains(&y) {
            break;
        }
    }

    yValues.push(y);


let poly = DensePolynomial::new(yValues);

let mut shares: Vec<(F, F)> = Vec::new();

for _i in 0..quorum {
    let mut x: F = F::rand(&mut rng);
    while x ==F:zero(){
        x = F::rand(&mut rng);
    }

    let y = poly.evaluate(x);
    shares.push((x,y));
}
shares

}


fn get_secret<F: PrimeField>(secret_shares: Vec<(F,F)>) -> F {
    let xValues: Vec<F> = secret_shares.iter().map(|(x, _)| *x).collect();
    let yValues:Vec<F> = secret_shares.iter().map(|(_, y)| * y).collect();

    result.evaluate(F::zero())
}



tests {
    use super::*;
    use ark_bn254::Fq;
    use std::i32;

    fn return_values() -> (Fq, u64, u64) {
        let secret = Fq::from(i32::MAX);
        let quorum = 4;
        let shares = 10;
        (secret, quorum, share_number)
    }


    fn test_share_secret(){
        let(secret, quorum, share_number) = return_values();
        let shares = split_secret(secret, quorum, share_number);
        assert_eq!(shares.len(), shares_number as usize);
    }


    fn test_get_secret_success() {
        let (secret, quorum, share_number) = return_values();
        let shares = split_secret(secret, quorum, share_number);
        let first_shares: Vec<(Fq, Fq)> = shares.iter().take(4).cloned().collect();
        let generated_secret = get_secret(first_shares);
        assert_eq!(secret, generated_secret);
    }
}






fn main() {
    println!("Hello, world!");
}
