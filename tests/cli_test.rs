use assert_cmd::prelude::*;
use std::process::Command;

// `rct --help` should print help (also ensure Opt struct is valid)
#[test]
fn cli_help() {
  Command::cargo_bin("rct").unwrap().args(&["--help"]).assert().success();
}
