use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use i18n::i18n;

#[derive(Parser)]
struct Cli;

fn main() -> Result<()> { -> anyhow::Result<()> {
    let config = i18n::Config::default();
    let mut component = i18n::new(config);
    component.initialize()?;
    println!("SigmaOS-i18n ready");
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
