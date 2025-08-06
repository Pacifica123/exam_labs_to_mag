// src/polynomial.rs

use rand::Rng;
use crate::utils::write_vec_to_txt;

/// Мультиномиальное (полиномиальное) распределение без rand::distributions
pub fn polynomial_run(n: usize, probabilities: &[f64], trials: usize) {
    let k = probabilities.len();
    let mut sum_counts = vec![0.0; k];
    let mut all_trials: Vec<Vec<usize>> = Vec::with_capacity(trials);

    // Построение кумулятивных границ вероятностей вручную
    let mut cumulative = Vec::with_capacity(k);
    let mut total = 0.0;
    for &p in probabilities {
        total += p;
        cumulative.push(total);
    }

    for _ in 0..trials {
        let mut counts = vec![0; k];
        for _ in 0..n {
            let r = rand::thread_rng().gen_range(0.0..1.0);
            for (i, &threshold) in cumulative.iter().enumerate() {
                if r < threshold {
                    counts[i] += 1;
                    break;
                }
            }
        }
        for i in 0..k {
            sum_counts[i] += counts[i] as f64;
        }
        all_trials.push(counts);
    }

    // Запись первой симуляции
    let flattened: Vec<u64> = all_trials[0].iter().map(|&x| x as u64).collect();
    let title = format!(
        "Полиномиальное распределение (n = {}, trials = {}, k = {})",
        n, trials, k);
    let _ = write_vec_to_txt("output/polynomial.txt", &title, &flattened);

    println!("=== Polynomial Distribution ===");
    for i in 0..k {
        let empirical_mean = sum_counts[i] / trials as f64;
        let theoretical_mean = n as f64 * probabilities[i];
        let deviation = (empirical_mean - theoretical_mean).abs();
        println!(
            "Category {}: Empirical mean = {:.3}, Theoretical = {:.3}, Deviation = {:.5}",
            i + 1, empirical_mean, theoretical_mean, deviation
        );
    }
}
