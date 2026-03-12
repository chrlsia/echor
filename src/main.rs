use clap::Parser; // it is a trait, we need it

// it expands the capabilities of the following struct
#[derive(Debug,Parser)]
// turn the struct into a clap::Command object
// get values of author and version from Cargo.toml
// get value of about from /// below
#[command(author,version,about)]
/// Rust version of `echo`

// Define a struct called Args
// each member of struct becomes clap::Arg
struct Args{
    /// Input text
    // field text is required
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print newline
    // define a short flag -n for the following argument
    #[arg(short('n'))]
    omit_newline: bool,// default to false
}

fn main(){
    let args= Args::parse();
    dbg!(&args);
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline{""} else {"\n"}
    );
}

/*
cargo run -q -- Hello world
[src/main.rs:27:5] args = Args {
    text: [
        "Hello",
        "world",
    ],
    omit_newline: false,
}

cargo run -q -- -n Hello world
[src/main.rs:27:5] args = Args {
    text: [
        "Hello",
        "world",
    ],
    omit_newline: true,
}
*/