// src/paskal.rs

use rand::Rng;
use crate::utils::write_vec_to_txt;

/// Распределение Паскаля: Y — число всех испытаний до достижения n успехов
pub fn paskal_run(n: u64, p: f64, trials: usize) {
    let mut total_attempts = Vec::with_capacity(trials);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..trials {
        let mut attempts = 0;
        let mut successes = 0;
        while successes < n {
            attempts += 1;
            let r = rand::thread_rng().gen_range(0.0..1.0);
            if r < p {
                successes += 1;
            }
        }
        total_attempts.push(attempts);
        sum += attempts as f64;
        sum_sq += (attempts as f64).powi(2);
    }

    let filename = format!("output/paskal_{}.txt", n);
    let title = format!(
        "Распределение Паскаля (n = {}, p = {}, trials = {})",
        n, p, trials);
    let _ = write_vec_to_txt(&filename, &title, &total_attempts);

    let empirical_mean = sum / trials as f64;
    let empirical_var = (sum_sq / trials as f64) - empirical_mean.powi(2);

    let theoretical_mean = n as f64 / p;
    let theoretical_var = n as f64 * (1.0 - p) / (p * p);

    let mean_diff = (empirical_mean - theoretical_mean).abs();
    let var_diff = (empirical_var - theoretical_var).abs();

    println!("=== Paskal Distribution ===");
    println!("n = {}, p = {}, trials = {}", n, p, trials);
    println!("Empirical mean: {:.5}", empirical_mean);
    println!("Theoretical mean: {:.5} | Deviation: {:.5}", theoretical_mean, mean_diff);
    println!("Empirical variance: {:.5}", empirical_var);
    println!("Theoretical variance: {:.5} | Deviation: {:.5}", theoretical_var, var_diff);
}
