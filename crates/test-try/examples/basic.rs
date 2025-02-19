//! Basic tests for [test_try].

use derive_more::Display;
use derive_more::Error;
use derive_more::From;
use test_try::test_try;

fn main() {}

#[derive(Debug, Display, Error, From)]
#[display("Outer error")]
struct OuterError(InnerError);

#[derive(Debug, Display, Error)]
#[display("Inner error that actually explains what went wrong")]
struct InnerError;

#[test_try]
fn test_using_test_try() {
    Err(OuterError(InnerError))?;
}

#[test]
fn regular_rust_test_returning_result() -> Result<(), Box<dyn std::error::Error>> {
    Err(OuterError(InnerError))?;
    Ok(())
}

#[test]
fn regular_rust_test_with_unwrap() {
    #[allow(clippy::unnecessary_literal_unwrap)]
    Err::<(), _>(OuterError(InnerError)).unwrap()
}

#[test]
fn test_using_anyhow() -> anyhow::Result<()> {
    Err(OuterError(InnerError))?;
    Ok(())
}
