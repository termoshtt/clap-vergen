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
