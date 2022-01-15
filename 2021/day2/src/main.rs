use std::process;

use libday2::{
    run_part1,
    run_part2,
};
use utils::cmd::ask_user;

fn main() {
    let run_fn = ask_user(run_part1, run_part2).expect("Failed getting uesr input");

    if let Err(err) = run_fn() {
        eprintln!("Can I haz b0rken?!");
        eprintln!("{:#}", err);

        process::exit(1);
    }

    process::exit(0)
}
