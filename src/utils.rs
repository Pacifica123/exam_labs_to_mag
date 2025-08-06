use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Result;
use std::fs;
use std::io;
use std::path::Path;
use rand::Rng;
use std::fmt::Display;

pub fn generate_probabilities(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut probs: Vec<f64> = (0..n).map(|_| rng.gen_range(0.0..1.0)).collect();
    let sum: f64 = probs.iter().sum();
    for p in &mut probs {
        *p /= sum;
    }
    probs
}

pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }
    Ok(())
}


pub fn write_vec_to_txt<T: Display>(filename: &str, title: &str, data: &[T]) -> Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Записываем заголовок с #
    writeln!(writer, "#{}", title)?;

    // Записываем каждый элемент в отдельной строке
    for value in data {
        writeln!(writer, "{}", value)?;
    }

    writer.flush()?;
    Ok(())
}

