mod app_errors;
mod file_reader;
mod window;

use app_errors::AppError;
use window::Window;

pub fn run_part2() -> Result<(), AppError> {
  let data = file_reader::load_data()?;
  let mut increased: usize = 0;
  let mut window = Window::new();
  let mut cur_sum: u64;
  let mut prev_sum: u64;

  for depth in &data {
    window.slide(depth);

    if window.is_ready() {
      cur_sum = window.get_current_sum();
      prev_sum = window.get_previous_sum();

      if cur_sum > prev_sum {
        increased += 1;
      }
    }
  }

  println!("There are {} measurements larger than the previous", increased);

  Ok(())
}

pub fn run_part1() -> Result<(), AppError> {
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
