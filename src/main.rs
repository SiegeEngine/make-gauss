
fn main() {
    let sigma: f32 = (10.0_f32).sqrt(); // Sigma squared is variance.
    let mu: f32 = 0.0; // offset from zero -- expected value

    for x in 0..15 {
        println!("weight[{}] = {};", x, g(x as f32, sigma, mu));
    }
}

fn g(x: f32, sigma: f32, mu: f32) -> f32 {
    (1.0 / (sigma * (2.0 * ::std::f32::consts::PI).sqrt()))
        * ((-1.0/2.0) * ((x - mu)/sigma).powi(2)).exp()
}
