// src/hypergeometric.rs
use crate::utils::write_vec_to_txt;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn combinations(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    (1..=k).fold(1, |acc, i| acc * (n + 1 - i) / i)
}

pub fn hypergeometric_run(N: u64, D: u64, n: u64, trials: usize) {
    let mut successes = Vec::with_capacity(trials);
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..trials {
        let mut urn = vec![1; D as usize];
        urn.extend(vec![0; (N - D) as usize]);
        urn.shuffle(&mut thread_rng());

        let drawn = &urn[..n as usize];
        let count_black = drawn.iter().filter(|&&x| x == 1).count() as u64;

        successes.push(count_black);
        sum += count_black as f64;
        sum_sq += (count_black as f64).powi(2);
    }
    let filename = format!("output/hypergeometric_{}.txt", n);
    let title = format!(
        "Гипергеометрическое распределение (N = {}, D = {}, n = {}, попыток: {})", 
        N, D, n, trials);
    let _ = write_vec_to_txt(&filename,&title, &successes);

    let empirical_mean = sum / trials as f64;
    let empirical_var = (sum_sq / trials as f64) - empirical_mean.powi(2);

    let theoretical_mean = n as f64 * (D as f64) / (N as f64);
    let theoretical_var = (n as f64) * (D as f64 / N as f64) * (1.0 - D as f64 / N as f64)
        * ((N - n) as f64 / (N - 1) as f64);

    let mean_diff = (empirical_mean - theoretical_mean).abs();
    let var_diff = (empirical_var - theoretical_var).abs();

    println!("=== Hypergeometric Distribution ===");
    println!("N = {}, D = {}, n = {}, trials = {}", N, D, n, trials);
    println!("Empirical mean: {:.5}", empirical_mean);
    println!("Theoretical mean: {:.5} | Deviation: {:.5}", theoretical_mean, mean_diff);
    println!("Empirical variance: {:.5}", empirical_var);
    println!("Theoretical variance: {:.5} | Deviation: {:.5}", theoretical_var, var_diff);
}
