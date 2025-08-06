use crate::geometric::geometric_run;
use crate::hypergeometric::hypergeometric_run;
use crate::binomial::binomial_run;
use crate::negative_binomial::negative_binomial_run;
use crate::paskal::paskal_run;
use crate::polynomial::polynomial_run;
use crate::normal::normal_run;
use crate::utils::{generate_probabilities, remove_dir_contents};
use std::process::Command;

mod geometric;
mod utils;
mod hypergeometric;
mod binomial;
mod negative_binomial;
mod paskal;
mod polynomial;
mod normal;

fn main() {
    let clear_output = true; // Флаг рычаг
    if clear_output {
        remove_dir_contents("output").expect("Не удалось очистить папку output");
    }
    let trials = 1000;
    let n = 1000;
    let probabilities = generate_probabilities(n-800);

    // geometric_run(0.3, trials);
    // geometric_run(0.6, trials);
    // geometric_run(0.9, trials);
    // hypergeometric_run(1000, 300, 200, trials);
    // binomial_run(200, 0.3, trials);
    // negative_binomial_run(1, 0.5, trials);
    // negative_binomial_run(n, 0.5, trials);
    // negative_binomial_run(n, 0.5, trials);
    // paskal_run(1, 0.5, trials);
    // paskal_run(n, 0.5, trials);
    // polynomial_run(n, &probabilities, trials);
    normal_run(0.0, 1.0, 100000);
    normal_run(3.5, -2.0, 100000);

    Command::new("scripts/lvenv/bin/python3")
        .arg("scripts/plot_from_txt.py")
        .status()
        .expect("Не удалось запустить Python-скрипт");
}
