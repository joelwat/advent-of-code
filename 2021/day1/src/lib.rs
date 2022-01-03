mod app_errors;
mod file_reader;

use app_errors::AppError;

pub fn run() -> Result<(), AppError> {
  let data = file_reader::load_data()?;
  let mut increased: usize = 0;
  let mut prev: Option<&u64> = None;

  for depth in &data {
    // Check if this is the first iteration
    if prev.is_none() {
      prev = Some(depth);

      continue;
    }

    if depth > prev.unwrap() {
      increased += 1;
    }

    prev = Some(depth);
  }

  println!("There are {} measurements larger than the previous", increased);

  Ok(())
}
