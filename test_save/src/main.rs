// Simple script to save text. Removing truncate(true) in line 15 creates a bug where previous text remains on the file.
use std::fs::OpenOptions;
use std::io::Write;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let line = "Very long rust line.";
    let file = OpenOptions::new().create(true).write(true).open("output.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &line)?;
    writer.flush();

    let line = "Short line.";
    let file = OpenOptions::new().create(true).write(true).truncate(true).open("output.json")?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, &line)?;
    writer.flush()?;
    Ok(())
}
