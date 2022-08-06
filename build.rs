use anyhow::Result;
use vergen::{vergen, Config};

fn main() -> Result<()> {
    let mut cfg = Config::default();
    *cfg.sysinfo_mut().name_mut() = false;
    vergen(cfg)
}
