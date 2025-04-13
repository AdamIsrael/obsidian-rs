use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Verify that the file exists.
pub fn file_exists<P: AsRef<Path>>(filename: P) -> bool {
    let mut path = PathBuf::new();
    path.push(filename);
    path.is_file()
}

/// Read in the contents of the file to a String
pub fn slurp<P: AsRef<Path>>(filename: P) -> String {
    let mut input: io::BufReader<File> =
        io::BufReader::new(File::open(&filename).expect("didn't work"));
    let mut md = String::new();
    input.read_to_string(&mut md).unwrap_or_else(|_| {
        panic!(
            "can't read string from file {}",
            filename.as_ref().to_string_lossy()
        )
    });
    md
}

/// Read in the contents of the URL to a String
pub fn slurp_url(url: String) -> String {
    let mut body = String::new();
    if let Ok(resp) = reqwest::blocking::get(url) {
        body = resp.text().unwrap_or("".to_string())
    }
    body
}

// /// Get a temporary filename
// pub fn get_temp_filename() -> PathBuf {
//     let mut temp_dir = std::env::temp_dir();
//     temp_dir.push(uuid::Uuid::new_v4().to_string());
//     temp_dir
// }

/// Download a file from a URL to a file
/// If the file fails to download, nothing will be written to disk.
pub fn download_to_file(url: String, filename: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(resp) = reqwest::blocking::get(url) {
        if let Ok(bytes) = resp.bytes() {
            std::fs::write(filename, bytes)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;
}
