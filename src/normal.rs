// src/normal.rs

use rand::Rng;
use crate::utils::write_vec_to_txt;

/// Генерация нормальной случайной величины через ЦПТ (центральную предельную теорему)
pub fn normal_run(mu: f64, sigma: f64, trials: usize) {
    let mut values = Vec::with_capacity(trials);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;
    let mut rng = rand::thread_rng();

    for _ in 0..trials {
        // Сумма 12 равномерных - 6: центрируем до N(0,1)
        let mut uniform_sum = 0.0;
        for _ in 0..12 {
            uniform_sum += rng.gen_range(0.0..1.0);
        }
        let nu = uniform_sum - 6.0;
        let x = mu + sigma * nu;

        values.push(x);
        sum += x;
        sum_sq += x.powi(2);
    }

    let filename = format!("output/geometric_{}.txt", mu);
    let title = format!("Нормальное распределение (mu = {}, sigma = {}, trials = {})", mu, sigma, trials);
    let _ = write_vec_to_txt(&filename, &title, &values);

    let empirical_mean = sum / trials as f64;
    let empirical_var = (sum_sq / trials as f64) - empirical_mean.powi(2);

    println!("=== Normal Distribution (CLT method) ===");
    println!("mu = {}, sigma = {}, trials = {}", mu, sigma, trials);
    println!("Empirical mean: {:.5}", empirical_mean);
    println!("Empirical variance: {:.5}", empirical_var);
}
