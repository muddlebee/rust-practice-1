use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn read_file_contents(path: &str) -> Result<String> {
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .with_context(|| format!("Failed to read contents of file: {}", path))?;
    Ok(contents)
}

fn main() -> Result<()> {
    let contents = read_file_contents("hello.txt")
        .with_context(|| "Error occurred while reading 'hello.txt'")?;
    println!("File contents: {}", contents);
    Ok(())
}
