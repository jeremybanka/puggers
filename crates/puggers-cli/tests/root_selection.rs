use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn root_flag_selects_first_matching_region_from_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_puggers"))
        .arg("--root")
        .arg("html>body article")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("puggers binary should run");

    child
        .stdin
        .as_mut()
        .expect("stdin should be available")
        .write_all(
            b"<!doctype html><html><body><header>Top</header><main><section><article><h1>First</h1></article></section><article><h1>Second</h1></article></main></body></html>",
        )
        .expect("stdin should write");

    let output = child.wait_with_output().expect("puggers should finish");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout should be utf-8"),
        "article\n  h1 First\n"
    );
}

#[test]
fn root_flag_reports_missing_regions() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_puggers"))
        .arg("--root")
        .arg("html>body>article")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("puggers binary should run");

    child
        .stdin
        .as_mut()
        .expect("stdin should be available")
        .write_all(b"<!doctype html><html><body><main><article><h1>Nested</h1></article></main></body></html>")
        .expect("stdin should write");

    let output = child.wait_with_output().expect("puggers should finish");

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8(output.stderr).expect("stderr should be utf-8"),
        "root not found: html>body>article\n"
    );
    assert_eq!(
        String::from_utf8(output.stdout).expect("stdout should be utf-8"),
        ""
    );
}
