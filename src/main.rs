extern crate rand;

mod arrg;

use arrg::Arrg;
use std::process;

/// Print a list of passwords of given length.
fn print_list(length: u32, count: u32, allow_symbols: bool) {
    let mut alphabet = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ0123456789");

    if allow_symbols {
        alphabet.push_str("/.;!@$");
    }

    for _ in 0..count {
        let mut code = String::new();

        for _ in 0..length {
            let number = rand::random::<f32>() * (alphabet.len() as f32);
            let number = number.round() as usize;

            match alphabet.chars().nth(number) {
                Some(character) => code.push(character),
                None => {}
            }
        }

        println!(" {}", code);
    }
}

fn main() {
    let arguments = Arrg::new()
        .command("-a", "alphanumeric_only")
        .command("-c", "count")
        .command("-l", "length")
        .parse();

    let count: u32 = match arguments.get("count") {
        Some(count) => {
            count.parse().unwrap_or_else(|_| {
                eprintln!("Invalid value for 'count'");
                process::exit(1);
            })
        },
        None => 10
    };

    let length: u32 = match arguments.get("length") {
        Some(length) => {
            length.parse().unwrap_or_else(|_| {
                eprintln!("Invalid value for 'length'");
                process::exit(1);
            })
        },
        None => 16
    };

    let alphanumeric_only: bool = match arguments.get("alphanumeric_only") {
        Some(_) => true,
        None => false
    };

    print_list(length, count, !alphanumeric_only);
}
