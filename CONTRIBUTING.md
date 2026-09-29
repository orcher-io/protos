# Contributing

Thanks for your interest in improving the ORCHER protocol definitions.

## Reporting issues

Open a GitHub issue describing the problem or the change you would like,
with the RPC or message involved and what you expected.

## Making changes

These files define a wire protocol shared by the ORCHER engine and every
SDK, so changes must stay backward compatible:

- Never change the number or type of an existing field, and never reuse a
  number. When removing a field, add its number and name to a `reserved`
  statement.
- Add new fields, enum values and RPCs rather than changing existing ones.
  An unset new field must mean what clients that predate it expect.
- Document every new service, RPC, message, field and enum value. Comments in
  the `.proto` files are the API reference: say what a value means, what the
  server does with it, and what an empty or unset value means.

## Checking your change

You need a Rust toolchain; the build compiles the protos itself.

```bash
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Pull requests

Keep each pull request to one change, and describe what it adds and why.
By contributing, you agree that your contributions are licensed under the
Apache License, Version 2.0 (see [LICENSE](LICENSE)).
