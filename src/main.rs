use crate::geometric::geometric_run;
use crate::hypergeometric::hypergeometric_run;
use crate::binomial::binomial_run;
use crate::negative_binomial::negative_binomial_run;
use crate::paskal::paskal_run;
use std::process::Command;

mod geometric;
mod utils;
mod hypergeometric;
mod binomial;
mod negative_binomial;
mod paskal;

fn main() {
    println!("Hello, world!");

    // geometric_run(0.3, 10000);
    // geometric_run(0.6, 10000);
    // geometric_run(0.9, 10000);
    // hypergeometric_run(1000, 300, 200, 1000);
    // binomial_run(200, 0.3, 1000);
    negative_binomial_run(1, 0.5, 1000);
    // negative_binomial_run(10, 0.5, 1000);
    negative_binomial_run(30, 0.5, 1000);
    paskal_run(1, 0.5, 1000);
    paskal_run(30, 0.5, 1000);

    Command::new("scripts/lvenv/bin/python3")
        .arg("scripts/plot_from_txt.py")
        .status()
        .expect("Не удалось запустить Python-скрипт");
}
