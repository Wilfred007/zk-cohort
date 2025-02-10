use ark_ff::PrimeField;
use rand::Rng;
// extern crate univariate;
use shamir::univariate::DensePolynomial;

fn split_secret(secret: f64, n: u32, quorum: u32) -> Vec<(f64, f64)> {
    let mut points: Vec<(f64, f64)> = vec![(0.0, secret)];

    let mut rng = rand::thread_rng();

    for _i in 0..quorum - 1 {
        let x: f64 = rng.gen_range(1.0..10000000.0);
        let y: f64 = rng.gen_range(1.0..10000000.0);

        if y == secret {
            panic!("failed!");
        }

        points.push((x, y));

        dbg!(&points);
    }

    let poly = DensePolynomial::interpolate(&points);

    let mut shares: Vec<(f64, f64)> = Vec::new();

    for i in 1..=n {
        // let x: f64 = rng.gen_range(1.0..100.0);
        let x: f64 = i.into();

        dbg!(poly.evaluate(x));

        let y = poly.evaluate(x);
        shares.push((x, y));
    }

    shares
}

fn get_secret(secret_shares: Vec<(f64, f64)>) -> f64 {
    // let xValues: Vec<f64> = secret_shares.iter().map(|(x, _)| *x).collect();
    // let yValues: Vec<f64> = secret_shares.iter().map(|(_, y)| *y).collect();
    let result_poly = DensePolynomial::interpolate(&secret_shares);
    result_poly.evaluate(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use std::i32;

    fn return_values() -> (f64, u32, u32) {
        let secret = f64::MAX;
        let quorum = 4;
        let share_number = 10;
        (secret, quorum, share_number)
    }

    #[test]
    fn test_share_secret() {
        let secret = f64::MAX;
        let quorum = 4;
        let shares_no = 10;
        let shares = split_secret(secret, shares_no, quorum);

        dbg!(&shares);
        assert_eq!(shares.len(), shares_no as usize);
    }

    #[test]
    fn test_get_secret_success() {
        let (secret, quorum, share_number) = return_values();
        let shares = split_secret(secret, share_number, quorum);
        let first_shares: Vec<(f64, f64)> = shares.iter().take(4).cloned().collect();

        dbg!(&first_shares);
        let generated_secret = get_secret(first_shares);
        assert_eq!(secret, generated_secret);
    }
}

fn main() {
    println!("Hello, world!");
}
