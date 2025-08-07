// src/pareto.rs

use rand::Rng;
use crate::utils::write_vec_to_txt;

/// Генерация по распределению Парето (метод обратной функции)
pub fn pareto_run(alpha: f64, xm: f64, trials: usize) {
    let mut values = Vec::with_capacity(trials);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut rng = rand::thread_rng();

    for _ in 0..trials {
        let r: f64 = rng.gen_range(0.0..1.0);
        let x = xm / (1.0 - r).powf(1.0 / alpha); // F⁻¹(r)
        values.push(x);
        sum += x;
        sum_sq += x.powi(2);
    }

    let filename = format!("output/pareto_{}_{}.txt", alpha, xm);
    let title = format!("Распределение Парето (alpha = {}, xm = {}, trials = {})", alpha, xm, trials);
    let _ = write_vec_to_txt(&filename, &title, &values);

    let empirical_mean = sum / trials as f64;
    let empirical_var = (sum_sq / trials as f64) - empirical_mean.powi(2);

    let theoretical_mean = if alpha > 1.0 { (alpha * xm) / (alpha - 1.0) } else { f64::INFINITY };
    let theoretical_var = if alpha > 2.0 {
        (alpha * xm.powi(2)) / ((alpha - 1.0).powi(2) * (alpha - 2.0))
    } else {
        f64::INFINITY
    };

    println!("=== Pareto Distribution ===");
    println!("alpha = {}, xm = {}, trials = {}", alpha, xm, trials);
    println!("Empirical mean: {:.5}, Theoretical: {:.5}", empirical_mean, theoretical_mean);
    println!("Empirical variance: {:.5}, Theoretical: {:.5}", empirical_var, theoretical_var);
}
