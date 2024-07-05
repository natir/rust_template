//! Functional test check regression in help message

/* std use */

/* crate use */

/* project use */

const HELP: &[u8] = b"Usage: rust_template [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>   Input sequence
  -q, --quiet           Silence all output
  -v, --verbosity...    Verbose mode (-v, -vv, -vvv, etc)
  -T, --timestamp <TS>  Timestamp (sec, ms, ns, none)
  -h, --help            Print help
  -V, --version         Print version
";

#[test]
fn help() -> rust_template::error::Result<()> {
    let mut cmd = assert_cmd::Command::cargo_bin("rust_template")?;
    cmd.args(["-h"]);

    let assert = cmd.assert();

    assert.success().stderr(b"" as &[u8]).stdout(HELP);

    Ok(())
}
