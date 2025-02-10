use polynomial::univariate::DensePolynomial;

fn main() {
    let test_cases = vec![
        // Linear (Degree 1)
        vec![(0.0, 1.0), (2.0, 5.0)],
        
        // Quadratic (Degree 2)
        vec![(0.0, 1.0), (1.0, 2.0), (2.0, 5.0)],
        
        // Cubic (Degree 3)
        vec![(0.0, 1.0), (1.0, 3.0), (2.0, 5.0), (3.0, 4.0)],
        
        // Quartic (Degree 4)
        vec![(0.0, 1.0), (1.0, 3.0), (2.0, 0.0), (3.0, 4.0), (4.0, 1.0)],
    ];

    for (i, points) in test_cases.iter().enumerate() {
        println!("\nTest Case {} (Degree {})", i + 1, points.len() - 1);
        println!("Points: {:?}", points);
        
        let poly = DensePolynomial::interpolate(points);
        println!("Coefficients: {:?}", poly.coefficients);
        
        println!("Verification:");
        for &(x, y) in points {
            let evaluated = poly.evaluate(x);
            println!("f({}) = {:.6} (expected: {})", x, evaluated, y);
        }
        
        let x = points[0].0 + 0.5;
        println!("Intermediate point f({}) = {:.6}", x, poly.evaluate(x));
    }
}
