use numerical_regression::LinearRegressionSolver;

fn main() {
    let x = vec![0.6, 1.3, 2.3, 3.5, 4.7, 5.2, 6.4, 8.8, 11.3, 15.4];
    let y = vec![1.78, 3.2, 5.4, 8.0, 9.9, 10.6, 13.8, 18.4, 22.9, 30.1];

    let solver = LinearRegressionSolver::new(x, y);
    dbg!(solver.solve());
}
