pub struct LinearRegressionSolver {
    x: Vec<f32>,
    y: Vec<f32>
}

impl LinearRegressionSolver {
    pub fn new(x: Vec<f32>, y: Vec<f32>) -> LinearRegressionSolver {
        LinearRegressionSolver { x, y }
    }

    pub fn solve(&self) -> (f32, f32) {
        let mut a = rand::random::<f32>();
        let mut b = rand::random::<f32>();
        let n = self.x.len();
        let alpha = 0.01_f32;

        loop {
            let mut sum_x = 0_f32;
            let mut sum_y = 0_f32;

            for i in 0..n {
                let t = 2.0 * (self.y[i] - a - b * self.x[i]);
                sum_x += t;
                sum_y += t * self.x[i];
            }
            
            let last_a = a;
            let last_b = b;

            a += sum_x * alpha / n as f32;
            b += sum_y * alpha / n as f32;

            if (a - last_a).abs() < 0.00001 &&
                (b - last_b).abs() < 0.00001 {
                    break;
                }
        }

        (a, b)
    }
}