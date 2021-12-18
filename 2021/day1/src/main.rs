use std::process;

fn main() {
    if let Err(err) = run() {
        eprintln!("This program is b0rked");
        eprintln!("Error: {}", err);

        process::exit(1);
    }
}
