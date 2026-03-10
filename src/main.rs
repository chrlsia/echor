fn main() {
    println!("{:?}",std::env::args());
}
/*
cargo r Hello world -n
Args { inner: ["target/debug/echor", "Hello", "world", "-n"] }

-------------
cargo r -n Hello world
Usage: run [OPTIONS] [ARGS]...

For more information, try '--help'
*/