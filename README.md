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

```
$ ./target/debug/main version
Build Timestamp:     2022-08-06T08:16:05.843030928Z
Build Version:       0.1.0
Commit SHA:          f1af7e4b9fc58b7aa73b1e14a617d9a341a9880d
Commit Date:         2022-08-06T08:12:22Z
Commit Branch:       main
rustc Version:       1.63.0-beta.8
rustc Channel:       beta
rustc Host Triple:   x86_64-unknown-linux-gnu
rustc Commit SHA:    7410ebb8f69516d0034cc99793bc3dcbc84d4a9b
cargo Target Triple: x86_64-unknown-linux-gnu
cargo Profile:       debug
```

```
$ ./target/debug/main version --json
{
  "build_timestamp": "2022-08-06T08:16:05.843030928Z",
  "build_semver": "0.1.0",
  "rustc_channel": "beta",
  "rustc_commit_date": "2022-08-04",
  "rustc_commit_hash": "7410ebb8f69516d0034cc99793bc3dcbc84d4a9b",
  "rustc_host_triple": "x86_64-unknown-linux-gnu",
  "rustc_llvm_version": "14.0",
  "rustc_semver": "1.63.0-beta.8",
  "cargo_features": "default",
  "cargo_profile": "debug",
  "cargo_target_triple": "x86_64-unknown-linux-gnu",
  "git_branch": "main",
  "git_commit_timestamp": "2022-08-06T08:12:22Z",
  "git_semver": "0.1.0",
  "git_sha": "f1af7e4b9fc58b7aa73b1e14a617d9a341a9880d"
}
```

License
--------

© 2022 Toshiki Teramura (@termoshtt)

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
