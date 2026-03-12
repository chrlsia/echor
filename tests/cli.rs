use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use pretty_assertions::assert_eq;

fn run(args:&[&str],expected_file:&str)->Result<()>{
    let expected=fs::read_to_string(expected_file)?;
    let output=Command::cargo_bin("echor")?
        .args(args)
        .output()
        .expect("fail");

    let stdout=String::from_utf8(output.stdout).expect("Invalid UTF-8");
    assert_eq!(stdout,expected);
    Ok(())
}

#[test]
fn dies_no_args()->Result<()>{
    // mesa apo to current crate pare thn command echor
    let mut cmd=Command::cargo_bin("echor")?;
    // exv thn command sthn cmd,cmd
    // trejthn , assert
    // bebaivsou oti apetyxe, failure
    // afoy apetyxe,grace to apotelesma sto stderr, stderr
    // kai ayto poy grafthke sto stderr periexei thn
    // lejh Usage,predicate::str::contains("Usage")
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

// #[test]
// fn run()->Result<()>{
//     let mut cmd= Command::cargo_bin("echor")?;
//     cmd.arg("hello").assert().success();
//     Ok(())
// }

#[test]
fn hello1()->Result<()>{
    run(&["Hello there"],"tests/expected/hello1.txt")

}

#[test]
fn hello2()->Result<()>{
    run(&["Hello","there"],"tests/expected/hello2.txt")

}

#[test]
fn test_hello1_no_newline()->Result<()>{
    run(&["Hello there","-n"],"tests/expected/hello1.n.txt")
}

#[test]
fn test_hello2_no_newline()->Result<()>{
    run(&["-n","Hello","there"],"tests/expected/hello2.n.txt")
}

/*
cargo testdox
✔ dies no args
✔ hello1
✔ hello2
✔ test hello1 no newline
✔ test hello2 no newline
*/