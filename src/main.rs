extern crate rand;

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

        println!("    {}", code);
    }
}

fn main() {
    println!("Generating keys...");

    for length in vec![8, 10, 12, 16, 32] {
        println!("");
        println!("{} characters:", length);
        print_list(length, 5, true);
    }

    println!("");
    println!("32 characters (no symbols)");
    print_list(32, 5, false);
}
