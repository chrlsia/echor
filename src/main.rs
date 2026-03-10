use clap::Command;

fn main() {
    let _matches= Command::new("echor")
        .version("0.1.0")
        .author("Chris Siannas <xrsia@otenet.gr>")
        .about("Rust version of `echo`")
        .get_matches();

}
/*
cargo run -q -- -h
Rust version of `echo`

Usage: echor

Options:
  -h, --help     Print help
  -V, --version  Print version
*/