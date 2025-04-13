use std::{
    fs::File,
    io::BufRead,
    path::Path,
};

use anyhow::Result;
use clap::{
    Arg,
    command,
};

fn main() -> Result<()> {
    let matches = command!()
        .arg(Arg::new("input").required(true))
        .get_matches();

    let input = matches
        .get_one::<String>("input")
        .expect("Input argument is required");

    // Read the input file and parse the numbers into two lists
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    let path = Path::new(input);
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;

        let mut buff = String::new();

        for char in line.chars() {
            if char.is_ascii_digit() {
                buff.push(char);
            } else if char.is_ascii_whitespace() {
                if buff.is_empty() {
                    // Skip consecutive whitespace
                    continue;
                }

                let num = buff.trim().parse::<u32>();

                match num {
                    Ok(n) => {
                        list1.push(n);
                        buff.clear();
                    },
                    Err(_) => {
                        eprintln!("Warning: Failed to parse number from '{}'", buff);
                        buff.clear();
                        continue;
                    },
                }
            }
        }

        let num = buff
            .trim()
            .parse::<u32>()
            .unwrap_or_else(|_| panic!("Warning: Failed to parse number from '{}'", buff));

        list2.push(num);
    }

    let mut similarity: u32 = 0;

    list1.iter().for_each(|a| {
        let in_list = find_in_list(&list2, *a);

        similarity += in_list * a;
    });

    println!("Similarity: {}", similarity);

    Ok(())
}

fn find_in_list(list: &Vec<u32>, num: u32) -> u32 {
    let mut count = 0;

    for &n in list {
        if n == num {
            count += 1;
        }
    }

    count
}
