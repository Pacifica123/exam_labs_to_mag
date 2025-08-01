use crate::geometric::geometric_run;
use crate::hypergeometric::hypergeometric_run;

mod geometric;
mod utils;
mod hypergeometric;

fn main() {
    println!("Hello, world!");

    // geometric_run(0.4, 100000);
    hypergeometric_run(10, 3, 5, 100);
}
