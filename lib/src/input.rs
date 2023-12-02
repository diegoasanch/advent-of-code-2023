use std::fs;

/// Read the contents of a file into a string.
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

/// Read the contents of a file into a vector of strings, one per line.
pub fn read_file_lines(path: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = read_file(path)?;
    Ok(contents.lines().map(|s| s.to_string()).collect())
}
