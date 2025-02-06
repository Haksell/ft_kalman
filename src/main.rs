use nalgebra::{Matrix3, Vector3};

struct KalmanFilter {
    state: Vector3<f64>,             // [x, y, z]
    covariance: Matrix3<f64>,        // Uncertainty
    transition_matrix: Matrix3<f64>, // Motion model
    process_noise: Matrix3<f64>,     // Noise in acceleration
}

impl KalmanFilter {
    fn predict(&mut self, dt: f64, acceleration: Vector3<f64>) {
        self.state = self.transition_matrix * self.state + dt * acceleration;
        self.covariance =
            self.transition_matrix * self.covariance * self.transition_matrix.transpose()
                + self.process_noise;
    }
}

impl KalmanFilter {
    fn update(&mut self, measurement: Vector3<f64>, measurement_noise: Matrix3<f64>) {
        let kalman_gain =
            self.covariance * (self.covariance + measurement_noise).try_inverse().unwrap();
        self.state = self.state + kalman_gain * (measurement - self.state);
        self.covariance = (Matrix3::identity() - kalman_gain) * self.covariance;
    }
}

fn main() {
    println!("Hello, world!");
}
