use assert_cmd::cmd::Command;

#[test]
fn test_debug_mode() {
    let result = Command::cargo_bin("exchan")
        .unwrap()
        .arg("--debug")
        .write_stdin("exit")
        .unwrap();

    assert_eq!(result.status.code(), Some(0));
}
