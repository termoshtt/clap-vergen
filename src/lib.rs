//! Reusable clap subcommand `version` using [vergen](https://crates.io/crates/vergen)
//!
//! ```no_run
//! use clap::Parser;
//! use clap_vergen::Version;
//!
//! #[derive(Debug, clap::Parser)]
//! enum Cli {
//!     Version(Version),
//! }
//!
//! fn main() {
//!     match Cli::from_args() {
//!         Cli::Version(version) => {
//!             version.print().unwrap();
//!         }
//!     }
//! }
//! ```

use anyhow::Result;
use serde::Serialize;
use std::fmt;

/// Infomations gathered by the default setting of
/// [vergen::Config](https://docs.rs/vergen/latest/vergen/struct.Config.html)
#[derive(Debug, Clone, Serialize)]
pub struct VergenInfo {
    /// `VERGEN_BUILD_TIMESTAMP` e.g. `2021-02-25T23:28:39.493201+00:00`
    pub build_timestamp: String,
    /// `VERGEN_BUILD_SEMVER` e.g. `5.0.0`
    pub build_semver: String,
    /// `VERGEN_RUSTC_CHANNEL` e.g. `nightly`
    pub rustc_channel: String,
    /// `VERGEN_RUSTC_COMMIT_DATE` e.g. `2021-02-24`
    pub rustc_commit_date: String,
    /// `VERGEN_RUSTC_COMMIT_HASH` e.g. `a8486b64b0c87dabd045453b6c81500015d122d6`
    pub rustc_commit_hash: String,
    /// `VERGEN_RUSTC_HOST_TRIPLE` e.g. `x86_64-apple-darwin`
    pub rustc_host_triple: String,
    /// `VERGEN_RUSTC_LLVM_VERSION` e.g. `11.0`
    pub rustc_llvm_version: String,
    /// `VERGEN_RUSTC_SEMVER` e.g. `1.52.0-nightly`
    pub rustc_semver: String,
    /// `VERGEN_CARGO_FEATURES` e.g. `git,build`
    pub cargo_features: String,
    /// `VERGEN_CARGO_PROFILE` e.g. `debug`
    pub cargo_profile: String,
    /// `VERGEN_CARGO_TARGET_TRIPLE` e.g. `x86_64-unknown-linux-gnu`
    pub cargo_target_triple: String,
    /// `VERGEN_SYSINFO_NAME` e.g. `Manjaro Linux`
    pub sysinfo_name: String,
    /// `VERGEN_SYSINFO_OS_VERSION` e.g. `Linux Manjaro Linux`
    pub sysinfo_os_version: String,
    /// `VERGEN_SYSINFO_USER` e.g. `Yoda`
    pub sysinfo_user: String,
    /// `VERGEN_SYSINFO_TOTAL_MEMORY` e.g. `33 GB`
    pub sysinfo_total_memory: String,
    /// `VERGEN_SYSINFO_CPU_VENDOR` e.g. `Authentic AMD`
    pub sysinfo_cpu_vendor: String,
    /// `VERGEN_SYSINFO_CPU_CORE_COUNT` e.g. `8`
    pub sysinfo_cpu_core_count: String,
    /// `VERGEN_SYSINFO_CPU_NAME` e.g. `cpu0,cpu1,cpu2,cpu3,cpu4,cpu5,cpu6,cpu7`
    pub sysinfo_cpu_name: String,
    /// `VERGEN_SYSINFO_CPU_BRAND` e.g. `AMD Ryzen Threadripper 1900X 8-Core Processor`
    pub sysinfo_cpu_brand: String,
    /// `VERGEN_SYSINFO_CPU_FREQUENCY` e.g. `3792`
    pub sysinfo_cpu_frequency: String,
    /// `VERGEN_GIT_BRANCH` e.g. `feature/fun`
    pub git_branch: String,
    /// `VERGEN_GIT_COMMIT_TIMESTAMP` e.g. `2021-02-24T20:55:21+00:00`
    pub git_commit_timestamp: String,
    /// `VERGEN_GIT_SEMVER` e.g. `5.0.0-2-gf49246c`
    pub git_semver: String,
    /// `VERGEN_GIT_SHA` e.g. `f49246ce334567bff9f950bfd0f3078184a2738a`
    pub git_sha: String,
}

impl Default for VergenInfo {
    fn default() -> Self {
        Self {
            build_semver: env!("VERGEN_BUILD_SEMVER").to_string(),
            build_timestamp: env!("VERGEN_BUILD_TIMESTAMP").to_string(),
            rustc_channel: env!("VERGEN_RUSTC_CHANNEL").to_string(),
            rustc_commit_date: env!("VERGEN_RUSTC_COMMIT_DATE").to_string(),
            rustc_commit_hash: env!("VERGEN_RUSTC_COMMIT_HASH").to_string(),
            rustc_host_triple: env!("VERGEN_RUSTC_HOST_TRIPLE").to_string(),
            rustc_llvm_version: env!("VERGEN_RUSTC_LLVM_VERSION").to_string(),
            rustc_semver: env!("VERGEN_RUSTC_SEMVER").to_string(),
            cargo_features: env!("VERGEN_CARGO_FEATURES").to_string(),
            cargo_profile: env!("VERGEN_CARGO_PROFILE").to_string(),
            cargo_target_triple: env!("VERGEN_CARGO_TARGET_TRIPLE").to_string(),
            sysinfo_cpu_brand: env!("VERGEN_SYSINFO_CPU_BRAND").to_string(),
            sysinfo_cpu_core_count: env!("VERGEN_SYSINFO_CPU_CORE_COUNT").to_string(),
            sysinfo_cpu_frequency: env!("VERGEN_SYSINFO_CPU_FREQUENCY").to_string(),
            sysinfo_cpu_name: env!("VERGEN_SYSINFO_CPU_NAME").to_string(),
            sysinfo_cpu_vendor: env!("VERGEN_SYSINFO_CPU_VENDOR").to_string(),
            sysinfo_name: env!("VERGEN_SYSINFO_NAME").to_string(),
            sysinfo_os_version: env!("VERGEN_SYSINFO_OS_VERSION").to_string(),
            sysinfo_total_memory: env!("VERGEN_SYSINFO_TOTAL_MEMORY").to_string(),
            sysinfo_user: env!("VERGEN_SYSINFO_USER").to_string(),
            git_branch: env!("VERGEN_GIT_BRANCH").to_string(),
            git_commit_timestamp: env!("VERGEN_GIT_COMMIT_TIMESTAMP").to_string(),
            git_semver: env!("VERGEN_GIT_SEMVER").to_string(),
            git_sha: env!("VERGEN_GIT_SHA").to_string(),
        }
    }
}

impl fmt::Display for VergenInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Build Timestamp:     2021-02-23T20:14:46.558472672+00:00
        writeln!(f, "{:<20} {}", "Build Timestamp:", self.build_timestamp)?;
        // Build Version:       0.1.0-9-g46f83e1
        writeln!(f, "{:<20} {}", "Build Version:", self.build_semver)?;
        // Commit SHA:          46f83e112520533338245862d366f6a02cef07d4
        writeln!(f, "{:<20} {}", "Commit SHA:", self.git_sha)?;
        // Commit Date:         2021-02-23T08:08:02-05:00
        writeln!(f, "{:<20} {}", "Commit Date:", self.git_commit_timestamp)?;
        // Commit Branch:       master
        writeln!(f, "{:<20} {}", "Commit Branch:", self.git_branch)?;
        // rustc Version:       1.52.0-nightly
        writeln!(f, "{:<20} {}", "rustc Version:", self.rustc_semver)?;
        // rustc Channel:       nightly
        writeln!(f, "{:<20} {}", "rustc Channel:", self.rustc_channel)?;
        // rustc Host Triple:   x86_64-unknown-linux-gnu
        writeln!(f, "{:<20} {}", "rustc Host Triple:", self.rustc_host_triple)?;
        // rustc Commit SHA:    3f5aee2d5241139d808f4fdece0026603489afd1
        writeln!(f, "{:<20} {}", "rustc Commit SHA:", self.rustc_commit_hash)?;
        // cargo Target Triple: x86_64-unknown-linux-musl
        writeln!(
            f,
            "{:<20} {}",
            "cargo Target Triple:", self.cargo_target_triple
        )?;
        // cargo Profile:       release
        writeln!(f, "{:<20} {}", "cargo Profile:", self.cargo_profile)?;
        Ok(())
    }
}

impl VergenInfo {
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Output detail version info
///
/// See crate level document.
#[derive(clap::Args, Debug, Clone)]
pub struct Version {
    /// Output version info as JSON
    #[clap(long)]
    json: bool,
}

impl Version {
    pub fn print(&self) -> Result<()> {
        let info = VergenInfo::default();
        if self.json {
            println!("{}", info.to_json()?);
        } else {
            println!("{}", info);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let info = VergenInfo::default();
        println!("{}", info);
    }

    #[test]
    fn json() {
        let info = VergenInfo::default();
        println!("{}", info.to_json().unwrap());
    }

    #[test]
    fn verify_app() {
        #[derive(Debug, clap::Parser)]
        struct Cli {
            #[clap(flatten)]
            version: Version,
        }
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
