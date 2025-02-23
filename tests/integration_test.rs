use assert_cmd::Command;

#[test]
fn test_should_display_first_line() {
    let mut cmd = Command::cargo_bin("traceroute").unwrap();

    cmd.args(["-m", "20", "localhost", "80"])
        .assert()
        .code(0)
        .stdout(predicates::str::contains(
            "Traceroute to localhost (127.0.0.1), 20 hops max, 80 byte packets",
        ));
}
