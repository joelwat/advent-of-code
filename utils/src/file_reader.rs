use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::app_errors::AppError;

pub fn load_data() -> Result<Vec<u64>, AppError> {
    if Path::new("src/input.txt").exists() {
        let file = File::open("src/input.txt")?;

        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut data: Vec<u64> = Vec::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    // Check for EOF condition
                    if bytes_read == 0 {
                        break;
                    }

                    data.push(line.trim().parse::<u64>().unwrap());

                    line.clear();
                }
                Err(err) => {
                    return Err(AppError::IoError(err));
                }
            }
        }

        return Ok(data);
    }

    Err(AppError::FileNotFound)
}
