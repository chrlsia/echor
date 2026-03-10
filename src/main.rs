use clap::Command;

fn main() {
    let _matches= Command::new("echor")
        .version("0.1.0")
        .author("Chris Siannas <xrsia@otenet.gr>")
        .about("Rust version of `echo`")
        .get_matches();

}
