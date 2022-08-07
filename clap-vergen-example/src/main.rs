use clap::Parser;

#[derive(Debug, clap::Parser)]
enum Cli {
    Version(clap_vergen::Version),
}

fn main() {
    match Cli::from_args() {
        Cli::Version(version) => {
            clap_vergen::print!(version);
        }
    }
}
