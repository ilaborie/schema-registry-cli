# Schema registry CLI

[![Crates.io](https://img.shields.io/crates/v/schema-registry-cli.svg)](https://crates.io/crates/schema-registry-cli)
[![Documentation](https://docs.rs/schema-registry-cli/badge.svg)](https://docs.rs/schema-registry-cli/)
[![Codecov](https://codecov.io/github/ilaborie/schema-registry-cli/coverage.svg?branch=main)](https://codecov.io/gh/ilaborie/schema-registry-cli)
[![Dependency status](https://deps.rs/repo/github/ilaborie/schema-registry-cli/status.svg)](https://deps.rs/repo/github/ilaborie/schema-registry-cli)

Provide a CLI to call with a schema registry.

## Install

### From source

If you have the [Rust tooling](https://rustup.rs/)


```bash
$ cargo install --locked schema-registry-cli
```

### From pre-built binaries

TODO


## Usage

### Show help

```console
$ schema-registry-cli --help
CLI for a schema-registry

Usage: schema-registry-cli [OPTIONS] <COMMAND>

Commands:
  subject     Subject commands
  schema      Schema commands
  completion  Generate shell completions
  help        Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  More outputs per occurrence
  -q, --quiet...    Less outputs per occurrence
  -h, --help        Print help
  -V, --version     Print version

```

### Subject commands

```console
$ schema-registry-cli subject --help
Subject commands

Usage: schema-registry-cli subject [OPTIONS] <COMMAND>

Commands:
  list      List subjects
  register  Register schema
  check     Check schema compatibility
  delete    Delete subject
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  More outputs per occurrence
  -q, --quiet...    Less outputs per occurrence
  -h, --help        Print help

```

### Schema commands

```console
$ schema-registry-cli schema --help
Schema commands

Usage: schema-registry-cli schema [OPTIONS] <COMMAND>

Commands:
  get   List subjects
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  More outputs per occurrence
  -q, --quiet...    Less outputs per occurrence
  -h, --help        Print help

```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.