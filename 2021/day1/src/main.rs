use std::process;

#[allow(unused_imports)]
use libday1::{
    run_part1,
    run_part2,
};

fn main() {
    if let Err(err) = run_part2() {
        eprintln!("Can I haz b0rken?!");
        eprintln!("{:#}", err);

        process::exit(1);
    }

    process::exit(0)
}
