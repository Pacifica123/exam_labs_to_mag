
// src/geometric.rs

use rand::Rng;
use std::f64;
use crate::utils::write_vec_to_txt;

pub fn geometric_run(p: f64, n: usize) {
    let mut results = Vec::with_capacity(n);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..n {
        let r: f64 = rand::thread_rng().gen_range(0.0..1.0);
        let y = ((r.ln() / (1.0 - p).ln()).floor() + 1.0) as u64;
        results.push(y);
        sum += y as f64;
        sum_sq += (y as f64).powi(2);
    }
    let filename = format!("output/geometric_{}.txt", p);
    let title = format!("Геометрическое распределение (p = {}, n = {})", p, n);
    let _ = write_vec_to_txt(&filename, &title, &results);

    let empirical_mean = sum / n as f64;
    let empirical_var = (sum_sq / n as f64) - empirical_mean.powi(2);

    let theoretical_mean = 1.0 / p;
    let theoretical_var = (1.0 - p) / (p * p);

    let mean_diff = (empirical_mean - theoretical_mean).abs();
    let var_diff = (empirical_var - theoretical_var).abs();

    println!("=== Geometric Distribution (Ferri version) ===");
    println!("p = {} | n = {}", p, n);
    println!("Empirical mean: {:.5}", empirical_mean);
    println!("Theoretical mean: {:.5} | Deviation: {:.5}", theoretical_mean, mean_diff);
    println!("Empirical variance: {:.5}", empirical_var);
    println!("Theoretical variance: {:.5} | Deviation: {:.5}", theoretical_var, var_diff);
}
