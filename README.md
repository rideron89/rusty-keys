# rusty-keys

> Random keygen built with [Rust](https://www.rust-lang.org/).

## Usage

Run with `cargo`:

```bash
# With no arguments, defaults to 10 keys of 16 characters.
cargo run

# With arguments, prints 5 keys of 12 characters with no symbols
cargo run -- -a -c 5 -l 12
```

Or compile and run as a stand-alone application:

```bash
cargo build --release

target/release/rusty-keys -l 32
```

### Screenshot

<img src="https://raw.githubusercontent.com/rideron89/rusty-keys/master/screenshot.png">

## Arguments

The following command arguments are accepted:

```bash
-a    Only use alphanumeric characters (default: false)
-c    Number of keys to generate       (default: 10)
-l    Character length of each key     (default: 16)
-t    Template                         (default: none)
```

## Templates
