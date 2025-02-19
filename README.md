# `#[test_try]`

An alternative to Rust's `#[test]` macro for writing unit tests.

## Usage

```toml
[dependencies]
test-try = "0.1"
```

## Introduction

Rust unit test typically handle errors by unwrapping or by returning a result with a boxed error:

```rust
/// Test using `.unwrap()` for error handling.
#[test]
fn regular_rust_test_with_unwrap() {
    Err::<(), _>(OuterError(InnerError)).unwrap();
}

/// Test returning `Result<(), Box<dyn std::error::Error>>` for error handling.
#[test]
fn regular_rust_test_returning_result() -> Result<(), Box<dyn std::error::Error>> {
    Err(OuterError(InnerError))?;
    Ok(())
}

#[derive(Debug, Display, Error, From)]
#[display("Outer error")]
struct OuterError(InnerError);

#[derive(Debug, Display, Error)]
#[display("Inner error that actually explains what went wrong")]
struct InnerError;
```

where the `cargo test` output may look like this

```
---- regular_rust_test_with_unwrap stdout ----
thread 'regular_rust_test_with_unwrap' panicked at crates/test-try/examples/basic.rs:32:42:
called `Result::unwrap()` on an `Err` value: OuterError(InnerError)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

or this

```
---- regular_rust_test_returning_result stdout ----
Error: OuterError(InnerError)
```

Critically, none of them include the source error, and writing `.unwrap()`, `-> Result<(), Box<dyn std::error::Error>>` and `Ok(())` is tedious.

`#[test_try]` is an alternative test macro:

```rust
#[test_try]
fn test_using_test_try() {
    Err(OuterError(InnerError))?;
}
```

with the following features:

- Use `?` to return errors without specifying a return type or ending the test with `Ok(())`.
- On error, the test prints out a report including the chain of source errors.

The `cargo test` output includes the full chain of source errors, in this case:

```
---- test_using_test_try stdout ----
Error: Outer error

Caused by:
      Inner error that actually explains what went wrong
```
