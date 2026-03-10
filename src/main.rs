use clap::{Arg,ArgAction,Command};

fn main() {
    let _matches= Command::new("echor")
        .version("0.1.0")
        .author("Chris Siannas <xrsia@otenet.gr>")
        .about("Rust version of `echo`")
        .arg(
            Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
            .short('n')
            .action(ArgAction::SetTrue)
            .help("Do not print newline"),
        )
        .get_matches();

        println!("{:#?}",_matches);
        
    // let text: Vec<&String> = matches
    //     .get_many::<String>("text")
    //     .unwrap()
    //     .collect();

    // let omit_newline = matches.get_flag("omit_newline");

    // println!("text: {:?}", text);
    // println!("omit_newline: {}", omit_newline);

}
