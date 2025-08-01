// src/negative_binomial.rs

use rand::Rng;
use crate::utils::write_vec_to_txt;

/// Отрицательное биномиальное распределение
/// x — число неудач до n успехов при вероятности успеха p
pub fn negative_binomial_run(n: u64, p: f64, trials: usize) {
    let mut failures = Vec::with_capacity(trials);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..trials {
        let mut x = 0; // число неудач
        let mut successes = 0;
        while successes < n {
            let r = rand::thread_rng().gen_range(0.0..1.0);
            if r < p {
                successes += 1;
            } else {
                x += 1;
            }
        }
        failures.push(x);
        sum += x as f64;
        sum_sq += (x as f64).powi(2);
    }

    let filename = format!("output/negative_binomial_{}.txt", n);
    let title = format!(
        "Отрицательное биномиальное распределение (n = {}, p = {}, trials = {})",
        n, p, trials);
    let _ = write_vec_to_txt(&filename, &title, &failures);

    let empirical_mean = sum / trials as f64;
    let empirical_var = (sum_sq / trials as f64) - empirical_mean.powi(2);

    let theoretical_mean = n as f64 * (1.0 - p) / p;
    let theoretical_var = n as f64 * (1.0 - p) / (p * p);

    let mean_diff = (empirical_mean - theoretical_mean).abs();
    let var_diff = (empirical_var - theoretical_var).abs();

    println!("=== Negative Binomial Distribution ===");
    println!("n = {}, p = {}, trials = {}", n, p, trials);
    println!("Empirical mean: {:.5}", empirical_mean);
    println!("Theoretical mean: {:.5} | Deviation: {:.5}", theoretical_mean, mean_diff);
    println!("Empirical variance: {:.5}", empirical_var);
    println!("Theoretical variance: {:.5} | Deviation: {:.5}", theoretical_var, var_diff);
}
