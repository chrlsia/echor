use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

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

#[test]
fn run()->Result<()>{
    let mut cmd= Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1()->Result<()>{
    let outfile="tests/expected/hello1.txt";
    let expected=fs::read_to_string(outfile)?;
    dbg!(&expected);
    let mut cmd=Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())

}

#[test]
fn hello2()->Result<()>{
    let outfile="tests/expected/hello2.txt";
    let expected=fs::read_to_string(outfile)?;
    dbg!(&expected);
    let mut cmd=Command::cargo_bin("echor")?;
    // here is .args 
    cmd.args(vec!["Hello","there"]).assert().success().stdout(expected);
    Ok(())

}
