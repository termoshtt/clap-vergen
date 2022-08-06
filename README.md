clap-vergen
============

Reusable clap subcommand `version` using [vergen](https://crates.io/crates/vergen)

```rust
use clap::Parser;
use clap_vergen::Version;

#[derive(Debug, clap::Parser)]
enum Cli {
    Version(Version),
}

fn main() {
    match Cli::from_args() {
        Cli::Version(version) => {
            version.print().unwrap();
        }
    }
}
```

License
--------

© 2022 Toshiki Teramura (@termoshtt)

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
