use std::process;

use libday1::run;

fn main() {
    if let Err(err) = run() {
        eprintln!("Can I haz b0rken?!");
        eprintln!("{:#}", err);

        process::exit(1);
    }

    process::exit(0)
}
