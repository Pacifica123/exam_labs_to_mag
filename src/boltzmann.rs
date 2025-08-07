// src/boltzmann.rs

use rand::Rng;
use crate::utils::write_vec_to_txt;

/// Генерация по распределению Больцмана через метод обратной функции
/// k — постоянная Больцмана, по умолчанию k = 1.0
pub fn boltzmann_run(T: f64, trials: usize) {
    const K: f64 = 1.380649;
    let mut values = Vec::with_capacity(trials);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut rng = rand::thread_rng();

    let lambda = 1.0 / (K * T);

    for _ in 0..trials {
        let r: f64 = rng.gen_range(0.0..1.0);
        let e = -r.ln() / lambda;
        values.push(e);
        sum += e;
        sum_sq += e.powi(2);
    }

    let filename = format!("output/boltzmann_{}.txt", T);
    let title = format!("Распределение Больцмана (T = {}, trials = {})", T, trials);
    let _ = write_vec_to_txt(&filename, &title, &values);

    let empirical_mean = sum / trials as f64;
    let empirical_var = (sum_sq / trials as f64) - empirical_mean.powi(2);
    let theoretical_mean = K * T;
    let theoretical_var = (K * T).powi(2);

    println!("=== Boltzmann Distribution ===");
    println!("T = {}, trials = {}", T, trials);
    println!("Empirical mean: {:.5}, Theoretical: {:.5}", empirical_mean, theoretical_mean);
    println!("Empirical variance: {:.5}, Theoretical: {:.5}", empirical_var, theoretical_var);
}
