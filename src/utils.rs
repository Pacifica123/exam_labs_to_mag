use std::fs::File;
use std::io::{BufWriter, Write};
use std::io::Result;

pub fn write_vec_to_txt(filename: &str, title: &str, data: &[u64]) -> Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Записываем заголовок с #
    writeln!(writer, "#{}", title)?;

    // Записываем каждый элемент в отдельной строке
    for &value in data {
        writeln!(writer, "{}", value)?;
    }

    writer.flush()?;
    Ok(())
}

