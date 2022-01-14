use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
    },
    path::Path,
};

use crate::app_errors::AppError;

pub fn load_data<T>(
    file_path: &Path,
    parse_line: &dyn Fn(&str) -> Result<T, AppError>,
) -> Result<Vec<T>, AppError> {
    if file_path.exists() {
        let file = File::open(file_path)?;

        let mut reader = BufReader::new(file);
        let mut line = String::new();
        let mut data: Vec<T> = Vec::new();

        loop {
            match reader.read_line(&mut line) {
                Ok(bytes_read) => {
                    // Check for EOF condition
                    if bytes_read == 0 {
                        break;
                    }

                    line = line.trim().to_string();

                    let parsed = parse_line(&line)?;
                    data.push(parsed);

                    line.clear();
                },
                Err(err) => {
                    return Err(AppError::IoError(err));
                },
            }
        }

        return Ok(data);
    }

    Err(AppError::FileNotFound)
}
