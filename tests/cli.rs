use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_no_args(){
    // mesa apo to current crate pare thn command echor
    let mut cmd=Command::cargo_bin("echor").unwrap();
    // exv thn command sthn cmd,cmd
    // trejthn , assert
    // bebaivsou oti apetyxe, failure
    // afoy apetyxe,grace to apotelesma sto stderr, stderr
    // kai ayto poy grafthke sto stderr periexei thn
    // lejh Usage,predicate::str::contains("Usage")
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn run(){
    let mut cmd= Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}