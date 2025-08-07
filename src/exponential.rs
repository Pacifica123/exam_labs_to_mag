// src/exponential.rs

use rand::Rng;
use crate::utils::write_vec_to_txt;

/// Генерация по экспоненциальному распределению через метод обратной функции
pub fn exponential_run(lambda: f64, trials: usize) {
    let mut values = Vec::with_capacity(trials);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut rng = rand::thread_rng();

    for _ in 0..trials {
        let r: f64 = rng.gen_range(0.0..1.0);
        let x = -r.ln() / lambda; // F⁻¹(r) = –ln(r)/λ
        values.push(x);
        sum += x;
        sum_sq += x.powi(2);
    }

    let filename = format!("output/exponential_{}.txt", lambda);
    let title = format!("Показательное распределение (lambda = {}, trials = {})", lambda, trials);
    let _ = write_vec_to_txt(&filename, &title, &values);

    let empirical_mean = sum / trials as f64;
    let empirical_var = (sum_sq / trials as f64) - empirical_mean.powi(2);
    let theoretical_mean = 1.0 / lambda;
    let theoretical_var = 1.0 / lambda.powi(2);

    println!("=== Exponential Distribution ===");
    println!("lambda = {}, trials = {}", lambda, trials);
    println!("Empirical mean: {:.5}, Theoretical: {:.5}", empirical_mean, theoretical_mean);
    println!("Empirical variance: {:.5}, Theoretical: {:.5}", empirical_var, theoretical_var);
}
