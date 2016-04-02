use std::io::Read;
use std::io::Write;

#[macro_use]
mod common;

use common::util::*;

static UTIL_NAME: &'static str = "tail";

static FOOBAR_TXT: &'static str = "foobar.txt";
static FOOBAR_WITH_NULL_TXT: &'static str = "foobar_with_null.txt";

#[test]
fn test_stdin_default() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let result = ucmd.run_piped_stdin(at.read(FOOBAR_TXT));
    assert_eq!(result.stdout, at.read("foobar_stdin_default.expected"));
}

#[test]
fn test_single_default() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let result = ucmd.arg(FOOBAR_TXT).run();
    assert_eq!(result.stdout, at.read("foobar_single_default.expected"));
}

#[test]
fn test_n_greater_than_number_of_lines() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let result = ucmd.arg("-n").arg("99999999").arg(FOOBAR_TXT).run();
    assert_eq!(result.stdout, at.read(FOOBAR_TXT));
}

#[test]
fn test_null_default() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let result = ucmd.arg("-z").arg(FOOBAR_WITH_NULL_TXT).run();
    assert_eq!(result.stdout, at.read("foobar_with_null_default.expected"));
}

#[test]
fn test_follow() {
    let (at, mut ucmd) = testing(UTIL_NAME);

    let mut child = ucmd.arg("-f").arg(FOOBAR_TXT).run_no_wait();

    let expected = at.read("foobar_single_default.expected");
    assert_eq!(read_size(&mut child, expected.len()), expected);

    // We write in a temporary copy of foobar.txt
    let expected = "line1\nline2\n";
    at.append(FOOBAR_TXT, expected);

    assert_eq!(read_size(&mut child, expected.len()), expected);

    child.kill().unwrap();
}

#[test]
fn test_single_big_args() {
    const FILE: &'static str = "single_big_args.txt";
    const EXPECTED_FILE: &'static str = "single_big_args_expected.txt";
    const LINES: usize = 1_000_000;
    const N_ARG: usize = 100_000;

    let (at, mut ucmd) = testing(UTIL_NAME);

    let mut big_input = at.make_scoped_file(FILE);
    for i in 0..LINES {
        write!(&mut big_input, "Line {}\n", i).expect("Could not write to FILE");
    }
    big_input.flush().expect("Could not flush FILE");

    let mut big_expected = at.make_scoped_file(EXPECTED_FILE);
    for i in (LINES - N_ARG)..LINES {
        write!(&mut big_expected, "Line {}\n", i).expect("Could not write to EXPECTED_FILE");
    }
    big_expected.flush().expect("Could not flush EXPECTED_FILE");

    let result = ucmd.arg(FILE).arg("-n").arg(format!("{}", N_ARG)).run();
    assert_eq!(result.stdout, at.read(EXPECTED_FILE));
}

#[test]
fn test_bytes_single() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let result = ucmd.arg("-c").arg("10").arg(FOOBAR_TXT).run();
    assert_eq!(result.stdout, at.read("foobar_bytes_single.expected"));
}

#[test]
fn test_bytes_stdin() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let result = ucmd.arg("-c").arg("13").run_piped_stdin(at.read(FOOBAR_TXT));
    assert_eq!(result.stdout, at.read("foobar_bytes_stdin.expected"));
}

#[test]
fn test_bytes_big() {
    const FILE: &'static str = "test_bytes_big.txt";
    const EXPECTED_FILE: &'static str = "test_bytes_big_expected.txt";
    const BYTES: usize = 1_000_000;
    const N_ARG: usize = 100_000;

    let (at, mut ucmd) = testing(UTIL_NAME);

    let mut big_input = at.make_scoped_file(FILE);
    for i in 0..BYTES {
        let digit = std::char::from_digit((i % 10) as u32, 10).unwrap();
        write!(&mut big_input, "{}", digit).expect("Could not write to FILE");
    }
    big_input.flush().expect("Could not flush FILE");

    let mut big_expected = at.make_scoped_file(EXPECTED_FILE);
    for i in (BYTES - N_ARG)..BYTES {
        let digit = std::char::from_digit((i % 10) as u32, 10).unwrap();
        write!(&mut big_expected, "{}", digit).expect("Could not write to EXPECTED_FILE");
    }
    big_expected.flush().expect("Could not flush EXPECTED_FILE");

    let result = ucmd.arg(FILE).arg("-c").arg(format!("{}", N_ARG)).run().stdout;
    let expected = at.read(EXPECTED_FILE);

    assert_eq!(result.len(), expected.len());
    for (actual_char, expected_char) in result.chars().zip(expected.chars()) {
        assert_eq!(actual_char, expected_char);
    }
}
